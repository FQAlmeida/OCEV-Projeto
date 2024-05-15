#[cfg(feature = "sequential")]
use std::iter::once;

#[cfg(not(feature = "sequential"))]
use rayon::iter::{
    once, IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

mod selection;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use loader_config::Config;
use log::info;
use population::{Individual, Population};
use problem_factory::problem::Problem;
use rand::{seq::SliceRandom, thread_rng, Rng};
use selection::{RouletteWheel, Selection, Tournament};

pub struct GA<'a> {
    config: &'a Config,
    problem: &'a (dyn Problem + Sync + Send),
    population: Population,
    best_individual: Option<Individual>,
    best_individual_value: Option<f64>,
    multi_progress_bar: &'a MultiProgress,
    generations_without_improvement: usize,
    generation: usize,
    selection_method: Box<dyn selection::Selection + Sync + Send>,
}

impl<'a> GA<'a> {
    pub fn new(
        problem: &'a (dyn Problem + Sync + Send),
        config: &'a Config,
        multi_progress_bar: &'a MultiProgress,
    ) -> Self {
        let population = Population::new(
            config.pop_config.pop_size,
            config.pop_config.dim,
            &config.pop_config.pop_type,
        );
        let selection_method: Box<dyn Selection + Sync + Send> =
            match config.selection_method {
                loader_config::SelectionMethod::Roulette => {
                    Box::new(RouletteWheel::new())
                }
                loader_config::SelectionMethod::Tournament => {
                    Box::new(Tournament::new(config.kp))
                }
            };
        GA {
            problem,
            config,
            population,
            multi_progress_bar,
            best_individual: None,
            best_individual_value: None,
            generations_without_improvement: 0,
            generation: 0,
            selection_method,
        }
    }

    fn evaluate(&self) -> Vec<(usize, f64)> {
        let population = &self.population.individuals;

        #[cfg(not(feature = "sequential"))]
        let population_iter = population.par_iter();
        #[cfg(feature = "sequential")]
        let population_iter = population.iter();

        population_iter
            .enumerate()
            .map(|(i, individual)| (i, self.problem.fitness(individual)))
            .collect()
    }

    fn update_best(&mut self, result: &[(usize, f64)]) -> Vec<(usize, f64)> {
        let mut new_result = result.to_vec();
        let (best_individual_index, best_individual_value_scaled) = new_result
            .par_iter()
            .max_by(|(_, a), (_, b)| {
                a.partial_cmp(b).expect("Failed to compare values.")
            })
            .expect("Failed to get best individual.");
        let (worst_individual_index, _) = new_result
            .par_iter()
            .min_by(|(_, a), (_, b)| {
                a.partial_cmp(b).expect("Failed to compare values.")
            })
            .expect("Failed to get worst individual.");
        let best_individual_value = *best_individual_value_scaled;
        if let Some(current_best) = self.best_individual_value {
            if best_individual_value >= current_best {
                self.generations_without_improvement = 0;
                self.best_individual_value = Some(best_individual_value);
                self.best_individual = Some(
                    self.population.individuals[*best_individual_index].clone(),
                );
            } else {
                self.generations_without_improvement += 1;
                if self.config.elitism {
                    self.population.individuals[*worst_individual_index] = self
                        .best_individual
                        .as_ref()
                        .expect("Unable to retrieve best individual")
                        .clone();
                    new_result = new_result
                        .iter()
                        .map(|tuple| {
                            if tuple.0 == *worst_individual_index {
                                return (
                                    *worst_individual_index,
                                    self.best_individual_value.expect(
                                        "Unable to retrieve best individual \
                                         value",
                                    ),
                                );
                            }
                            *tuple
                        })
                        .collect();
                }
            }
        } else {
            self.best_individual_value = Some(best_individual_value);
            self.best_individual = Some(
                self.population.individuals[*best_individual_index].clone(),
            );
        }
        new_result
    }

    fn genocide(&mut self) -> Vec<(usize, f64)> {
        self.generations_without_improvement = 0;
        let new_population = Population::new(
            self.config.pop_config.pop_size / 2,
            self.config.pop_config.dim,
            &self.config.pop_config.pop_type,
        );
        let mut indexes =
            (0..self.config.pop_config.pop_size).collect::<Vec<usize>>();
        let mut rng = thread_rng();
        indexes.shuffle(&mut rng);
        indexes
            .iter()
            .take(self.config.pop_config.pop_size / 2)
            .for_each(|&index| {
                let new_individual = new_population.individuals[index].clone();
                self.population.individuals[index] = new_individual;
            });
        let result = self.evaluate();

        self.update_best(&result)
    }

    fn selection(&self, result: &[(usize, f64)]) -> Vec<(usize, usize)> {
        self.selection_method.select(result)
    }

    fn crossover(&self, mating_pool: &[(usize, usize)]) -> Population {
        #[cfg(not(feature = "sequential"))]
        let mating_pool_iter = mating_pool.par_iter();
        #[cfg(feature = "sequential")]
        let mating_pool_iter = mating_pool.iter();

        let crossover_chance = self.config.crossover_chance;
        let couples_mapped = mating_pool_iter.map(|(parent1, parent2)| {
            let mut rng = rand::thread_rng();
            let crossover = rng.gen::<f64>();
            let child1 = &self.population.individuals[*parent1];
            let child2 = &self.population.individuals[*parent2];
            if crossover <= crossover_chance {
                return child1
                    .crossover(child2, &self.config.crossover_method);
            }
            (child1.clone(), child2.clone())
        });
        Population {
            individuals: couples_mapped
                .flat_map(|tuple| once(tuple.0).chain(once(tuple.1)))
                .collect(),
        }
    }

    fn mutation(&self, new_population: &Population) -> Population {
        #[cfg(not(feature = "sequential"))]
        let individuals_iter = new_population.individuals.par_iter();
        #[cfg(feature = "sequential")]
        let individuals_iter = new_population.individuals.iter();

        let mutation_chance = self.config.mutation_chance;
        let mutated_population = individuals_iter
            .map(|individual| individual.mutate(mutation_chance));

        Population {
            individuals: mutated_population.collect(),
        }
    }

    fn log_run_result(&self) {
        match &self.best_individual {
            Some(best_individual) => {
                info!("Best Individual: {}", best_individual.clone());
                info!(
                    "Best Individual Value: {}",
                    self.best_individual_value
                        .expect("Unable to retrieve best individual value")
                );
                info!(
                    "Best Individual Value Decoded: {}",
                    self.problem
                        .objective(&self.problem.decode(best_individual))
                );
                info!(
                    "Best Individual Decoded: {:?}",
                    self.problem.decode(best_individual)
                );
                info!(
                    "Best Individual Constraint: {}",
                    self.problem
                        .constraint(&self.problem.decode(best_individual))
                );
            }
            None => {}
        };
    }

    fn log_generation(&self, result: &[(usize, f64)]) {
        let generation = self.generation;
        let result_mapped = result.iter().map(|(_, value)| value);
        info!(
            "State Individual: {} {} {} {} {}",
            generation,
            self.best_individual_value
                .expect("Unable to retrieve best individual value"),
            result_mapped
                .clone()
                .max_by(|a, b| a.total_cmp(b))
                .unwrap_or(&0.0),
            result_mapped.clone().sum::<f64>() / result.len() as f64,
            result_mapped
                .clone()
                .min_by(|a, b| a.total_cmp(b))
                .unwrap_or(&0.0),
        );
    }

    fn linear_escalation(&self, result: &[(usize, f64)]) -> Vec<(usize, f64)> {
        let generation: f64 = self.generation as f64;
        let total_generations: f64 = self.config.qtd_gen as f64;
        let c = if generation < total_generations * 0.8 {
            1.2 + (((2.0 - 1.2) / (total_generations * 0.8)) * (generation))
        } else {
            2.0
        };
        let min = *result
            .par_iter()
            .map(|(_, value)| value)
            .min_by(|a, b| a.total_cmp(b))
            .expect("Failed to get min value.");
        let max = *result
            .par_iter()
            .map(|(_, value)| value)
            .max_by(|a, b| a.total_cmp(b))
            .expect("Failed to get max value.");
        let average = result.par_iter().map(|(_, value)| value).sum::<f64>()
            / result.len() as f64;
        let (alpha, beta) = if min > (c * average - max) / (c - 1.0) {
            (
                average * (c - 1.0) / (max - average),
                average * (max - c * average) / (max - average),
            )
        } else {
            (
                average / (average - min),
                (-min * average) / (average - min),
            )
        };
        result
            .par_iter()
            .map(|(index, value)| (*index, alpha * value + beta))
            .collect()
    }

    fn generation_gap(&self, new_population: &Population) -> Population {
        let generation: f64 = self.generation as f64;
        let total_generations: f64 = self.config.qtd_gen as f64;

        let proportion = if generation < total_generations * 0.8 {
            self.config.generation_gap
                + (((1.0 - self.config.generation_gap) / total_generations
                    * 0.8)
                    * generation)
        } else {
            1.0
        };
        let new_population_iter = new_population
            .individuals
            .par_iter()
            .zip(self.population.individuals.par_iter());
        let new_population = new_population_iter
            .map(|(new_individual, old_individual)| {
                let mut rng = rand::thread_rng();
                if rng.gen::<f64>() < proportion {
                    new_individual.clone()
                } else {
                    old_individual.clone()
                }
            })
            .collect();
        Population {
            individuals: new_population,
        }
    }

    fn check_genocide(
        &mut self,
        new_result: &[(usize, f64)],
    ) -> Vec<(usize, f64)> {
        if self.generations_without_improvement
            >= self.config.generations_to_genocide
        {
            self.genocide()
        } else {
            new_result.to_vec()
        }
    }

    /// # Panics
    /// If I did shit
    pub fn run(&mut self) -> (Option<Individual>, Option<f64>) {
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .expect("Failed to build progress bar template");
        let pb = self
            .multi_progress_bar
            .add(ProgressBar::new(self.config.qtd_gen as u64));
        pb.set_style(sty);

        for generation in 1..=self.config.qtd_gen {
            self.generation = generation;
            let result = self.evaluate();
            let new_result = self.update_best(&result);
            let newer_result = self.check_genocide(&new_result);

            self.log_generation(&newer_result);
            let scaled_result = self.linear_escalation(&new_result);
            let mating_pool = self.selection(&scaled_result);
            let mut new_population = self.crossover(&mating_pool);
            new_population = self.mutation(&new_population);

            self.population = self.generation_gap(&new_population);

            pb.inc(1);
        }
        self.log_run_result();
        pb.finish_with_message("Run completed");
        (self.best_individual.clone(), self.best_individual_value)
    }
}
