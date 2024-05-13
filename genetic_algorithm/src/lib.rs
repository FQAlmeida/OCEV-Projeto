#[cfg(feature = "sequential")]
use std::iter::once;

mod selection;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use individual_creation::{Individual, IndividualType, Population};
use loader_config::{Config, PopType};
use log::info;
use problem::Problem;
use rand::{rngs::OsRng, Rng};
use rand_unique::{RandomSequence, RandomSequenceBuilder};
#[cfg(not(feature = "sequential"))]
use rayon::iter::{
    once, IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use selection::{RouletteWheel, Selection, Tournament};

pub struct GA<'a> {
    pub config: &'a Config,
    pub problem: &'a (dyn Problem + Sync + Send),
    pub population: Population,
    pub best_individual: Option<Individual>,
    pub best_individual_value: Option<f64>,
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
        let individual_type = match config.pop_config.pop_type {
            PopType::Binary => IndividualType::Binary(vec![]),
            PopType::Real => todo!(),
            PopType::Integer => todo!(),
            PopType::Permuted => IndividualType::Permuted(vec![]),
        };
        let population = Population::new(
            config.pop_config.pop_size,
            config.pop_config.dim,
            &individual_type,
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
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        let (worst_individual_index, _) = new_result
            .iter()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
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
                    self.population.individuals[*worst_individual_index] =
                        self.best_individual.as_ref().unwrap().clone();
                    new_result = new_result
                        .iter()
                        .map(|tuple| {
                            if tuple.0 == *worst_individual_index {
                                return (
                                    *worst_individual_index,
                                    self.best_individual_value.unwrap(),
                                );
                            }
                            *tuple
                        })
                        .collect();
                }
            }
        } else {
            self.best_individual_value = Some(best_individual_value);
            self.best_individual =
                Some(self.population.individuals[*best_individual_index].clone());
        }
        new_result
    }

    fn genocide(&mut self) -> Vec<(usize, f64)> {
        self.generations_without_improvement = 0;
        let individual_type = match self.config.pop_config.pop_type {
            PopType::Binary => IndividualType::Binary(vec![]),
            PopType::Real => todo!(),
            PopType::Integer => todo!(),
            PopType::Permuted => IndividualType::Permuted(vec![]),
        };
        let new_population = Population::new(
            self.config.pop_config.pop_size / 2,
            self.config.pop_config.dim,
            &individual_type,
        );
        let config = RandomSequenceBuilder::<u16>::rand(&mut OsRng);
        let mut sequence: RandomSequence<u16> = config.into_iter();
        (0..self.config.pop_config.pop_size / 2)
            .map(|_| {
                (sequence.next().unwrap() as usize) % self.config.pop_config.pop_size
            })
            .for_each(|index| {
                self.population.individuals[index] = new_population.individuals
                    [index % (self.config.pop_config.pop_size / 2)]
                    .clone();
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
            let mut child1 = self.population.individuals[*parent1].clone();
            let mut child2 = self.population.individuals[*parent2].clone();
            if crossover <= crossover_chance {
                let crossover_point = rng.gen_range(0..self.config.pop_config.dim);
                (child1, child2) = child1.crossover(&child2, crossover_point);
            }
            (child1, child2)
        });
        let new_population: Population = Population {
            individuals: couples_mapped
                .flat_map(|tuple| once(tuple.0).chain(once(tuple.1)))
                .collect(),
        };

        new_population
    }

    fn mutation(&self, new_population: &Population) -> Population {
        #[cfg(not(feature = "sequential"))]
        let individuals_iter = new_population.individuals.par_iter();
        #[cfg(feature = "sequential")]
        let individuals_iter = new_population.individuals.iter();

        let mutation_chance = self.config.mutation_chance;
        let mutated_population =
            individuals_iter.map(|individual| individual.mutate(mutation_chance));

        Population {
            individuals: mutated_population.collect(),
        }
    }

    fn log_run_result(&self) {
        match &self.best_individual {
            Some(best_individual) => {
                info!("Best Individual: {}", best_individual.chromosome.clone());
                info!(
                    "Best Individual Value: {}",
                    self.best_individual_value.unwrap()
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
            self.best_individual_value.unwrap(),
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
            .unwrap();
        let max = *result
            .par_iter()
            .map(|(_, value)| value)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();
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
                + (((1.0 - self.config.generation_gap) / total_generations * 0.8)
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

    fn check_genocide(&mut self, new_result: &[(usize, f64)]) -> Vec<(usize, f64)> {
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
        .unwrap();
        let pb = self
            .multi_progress_bar
            .add(ProgressBar::new(self.config.qtd_gen.try_into().unwrap()));
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
