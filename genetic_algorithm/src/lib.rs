use std::fmt::{Display, Error, Formatter};

use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use individual_creation::{Individual, IndividualType, Population};
use loader_config::Config;
use log::info;
use problem::Problem;
use rand::{rngs::OsRng, Rng};
use rand_unique::{RandomSequence, RandomSequenceBuilder};
use rayon::iter::{once, IntoParallelRefIterator, ParallelIterator};

pub struct GA<'a> {
    pub config: &'a Config,
    pub problem: &'a Box<dyn Problem + Sync + Send>,
    pub population: Population,
    pub best_individual_index: Option<usize>,
    pub best_individual_value: Option<f64>,
    multi_progress_bar: &'a MultiProgress,
    generations_without_improvement: usize,
}

struct IndividualTypeVecDisplay(Vec<IndividualType>);

impl Display for IndividualTypeVecDisplay {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::from("[");

        for num in &self.0[0..self.0.len() - 1] {
            match num {
                IndividualType::Binary(value) => {
                    comma_separated.push_str(value.to_string().as_str());
                    comma_separated.push_str(", ");
                }
            }
        }

        comma_separated.push_str(&self.0[self.0.len() - 1].to_string().as_str());
        write!(f, "{}]", comma_separated)
    }
}

impl<'a> GA<'a> {
    pub fn new(
        problem: &'a Box<dyn Problem + Sync + Send>,
        config: &'a Config,
        multi_progress_bar: &'a MultiProgress,
    ) -> Self {
        let population = Population::new(
            config.pop_config.pop_size,
            config.pop_config.dim,
            &IndividualType::Binary(true),
        );
        GA {
            problem,
            config,
            population,
            multi_progress_bar,
            best_individual_index: None,
            best_individual_value: None,
            generations_without_improvement: 0,
        }
    }
    fn evaluate(&self) -> Vec<f64> {
        let population = &self.population.individuals;
        let fitness = population
            .par_iter()
            .map(|individual| self.problem.fitness(individual))
            .collect();
        return fitness;
    }

    fn update_best(&mut self, result: &Vec<f64>) -> Vec<f64> {
        let mut new_result = result.clone();
        let best_individual_index = result
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;
        let worst_individual_index = result
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;
        match self.best_individual_value {
            Some(value) => {
                if result[best_individual_index] >= value {
                    self.best_individual_value = Some(result[best_individual_index]);
                    self.best_individual_index = Some(best_individual_index);
                } else {
                    self.generations_without_improvement += 1;
                    if self.config.elitism {
                        self.population.individuals[worst_individual_index] =
                            self.best_individual().unwrap().clone();

                        new_result[worst_individual_index] = self.best_individual_value.unwrap();
                        self.best_individual_index = Some(worst_individual_index);
                    }
                }
            }
            None => {
                self.best_individual_value = Some(result[best_individual_index]);
                self.best_individual_index = Some(best_individual_index);
            }
        }
        return new_result;
    }

    fn best_individual(&self) -> Option<&Individual> {
        match self.best_individual_index {
            Some(index) => Some(&self.population.individuals[index]),
            None => None,
        }
    }

    fn genocide(&mut self) -> Vec<f64> {
        self.generations_without_improvement = 0;
        let new_population = Population::new(
            self.config.pop_config.pop_size / 2,
            self.config.pop_config.dim,
            &IndividualType::Binary(true),
        );
        let config = RandomSequenceBuilder::<u16>::rand(&mut OsRng);
        let mut sequence: RandomSequence<u16> = config.into_iter();
        (0..self.config.pop_config.pop_size / 2)
            .map(|_| (sequence.next().unwrap() as usize) % self.config.pop_config.pop_size)
            .for_each(|index| {
                self.population.individuals[index] = new_population.individuals
                    [index % (self.config.pop_config.pop_size / 2)]
                    .clone();
            });
        let result = self.evaluate();
        let new_result = self.update_best(&result);
        return new_result;
    }
    fn selection(&self, result: Vec<f64>) -> Vec<(usize, usize)> {
        let pop_size = self.config.pop_config.pop_size;
        let kp = self.config.kp;
        let mut rng = rand::thread_rng();
        let mut mating_pool: Vec<(usize, usize)> = Vec::with_capacity(pop_size / 2);
        for _ in 0..(pop_size / 2) {
            let parent1 = {
                let p1 = rng.gen_range(0..pop_size);
                let p2 = rng.gen_range(0..pop_size);
                let cmp_func = if rng.gen::<f64>() > kp {
                    |a: f64, b: f64| a < b
                } else {
                    |a: f64, b: f64| a > b
                };
                if cmp_func(result[p1], result[p2]) {
                    p1
                } else {
                    p2
                }
            };
            let parent2 = {
                let p1 = rng.gen_range(0..pop_size);
                let p2 = rng.gen_range(0..pop_size);
                let cmp_func = if rng.gen::<f64>() > kp {
                    |a: f64, b: f64| a < b
                } else {
                    |a: f64, b: f64| a > b
                };
                if cmp_func(result[p1], result[p2]) {
                    p1
                } else {
                    p2
                }
            };
            mating_pool.push((parent1, parent2));
        }
        return mating_pool;
    }

    fn crossover(&self, mating_pool: &Vec<(usize, usize)>) -> Population {
        // let mut new_population = Vec::with_capacity(self.config.pop_config.pop_size);
        let crossover_chance = self.config.crossover_chance;
        let couples_mapped = mating_pool.par_iter().map(|(parent1, parent2)| {
            let mut rng = rand::thread_rng();
            let crossover = rng.gen::<f64>();
            if crossover <= crossover_chance {
                let crossover_point = rng.gen_range(0..self.config.pop_config.dim);
                let mut child1 = self.population.individuals[*parent1].clone();
                let mut child2 = self.population.individuals[*parent2].clone();
                for i in 0..crossover_point {
                    child1.chromosome[i] = self.population.individuals[*parent2].chromosome[i];
                    child2.chromosome[i] = self.population.individuals[*parent1].chromosome[i];
                }
                return (child1, child2);
            }
            let child1 = self.population.individuals[*parent1].clone();
            let child2 = self.population.individuals[*parent2].clone();
            return (child1, child2);
        });
        let new_population: Population = Population {
            individuals: couples_mapped
                .flat_map(|tuple| once(tuple.0).chain(once(tuple.1)))
                .collect(),
        };
        return new_population;
    }

    fn mutation(&self, new_population: &Population) -> Population {
        let mutation_chance = self.config.mutation_chance;
        let mutated_population = new_population.individuals.par_iter().map(|individual| {
            let new_individual = individual.chromosome.iter().map(|gene| {
                let mut rng = rand::thread_rng();
                let mutation = rng.gen::<f64>();
                if mutation <= mutation_chance {
                    return gene.mutate();
                }
                return *gene;
            });
            return Individual {
                chromosome: new_individual.collect(),
            };
        });
        let population = Population {
            individuals: mutated_population.collect(),
        };
        return population;
    }

    fn log_run_result(&self) {
        // TODO(Otavio): Best Individual Value and Best Individual Value Decoded are not the same
        match self.best_individual() {
            Some(best_individual) => {
                info!(
                    "Best Individual: {}",
                    IndividualTypeVecDisplay(best_individual.chromosome.clone())
                );
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
    fn log_generation(&self, generation: usize, result: &Vec<f64>) {
        info!(
            "State Individual: {} {} {} {} {}",
            generation,
            self.best_individual_value.unwrap(),
            result.iter().max_by(|a, b| a.total_cmp(b)).unwrap_or(&0.0),
            result.iter().sum::<f64>() / result.len() as f64,
            result.iter().min_by(|a, b| a.total_cmp(b)).unwrap_or(&0.0),
        );
    }

    pub fn run(&mut self) -> (Option<&Individual>, Option<f64>) {
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap();
        let pb = self
            .multi_progress_bar
            .add(ProgressBar::new(self.config.qtd_gen.try_into().unwrap()));
        pb.set_style(sty);

        for generation in 1..=self.config.qtd_gen {
            let result = self.evaluate();
            let new_result = self.update_best(&result);
            let newer_result =
                if self.generations_without_improvement >= self.config.generations_to_genocide {
                    self.genocide()
                } else {
                    new_result
                };
            self.log_generation(generation, &newer_result);
            let mating_pool = self.selection(newer_result);
            let mut new_population = self.crossover(&mating_pool);
            new_population = self.mutation(&new_population);
            self.population = new_population;
            pb.inc(1)
        }
        self.log_run_result();
        pb.finish_with_message("Run completed");
        return (self.best_individual(), self.best_individual_value);
    }
}
