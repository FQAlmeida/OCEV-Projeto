use individual_creation::{Individual, IndividualType, Population};
use loader_config::Config;
use problem::Problem;
use rand::Rng;
use rayon::iter::{once, IntoParallelRefIterator, ParallelIterator};

pub struct GA<'a> {
    pub config: &'a Config,
    pub problem: &'a Box<dyn Problem + Sync + Send>,
    pub population: Population,
    pub best_individual_index: Option<usize>,
    pub best_individual_value: Option<f64>,
}

impl<'a> GA<'a> {
    pub fn new(problem: &'a Box<dyn Problem + Sync + Send>, config: &'a Config) -> Self {
        let population = Population::new(
            config.pop_config.pop_size,
            config.pop_config.dim,
            &IndividualType::Binary(true),
        );
        GA {
            problem,
            config,
            population,
            best_individual_index: None,
            best_individual_value: None,
        }
    }
    fn evaluate(&self) -> Vec<f64> {
        let population = self.population.individuals.clone();
        let fitness = population
            .par_iter()
            .map(|individual| self.problem.fitness(individual))
            .collect();
        return fitness;
    }

    fn update_best(&mut self, result: Vec<f64>) -> Vec<f64> {
        let mut new_result = result.clone();
        let best_individual_index = result
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;
        let worst_individual = result
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
                    if self.config.elitism {
                        self.population.individuals[worst_individual] =
                            self.best_individual().unwrap();
                        new_result[worst_individual] = self.best_individual_value.unwrap();
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

    fn best_individual(&self) -> Option<Individual> {
        match self.best_individual_index {
            Some(index) => Some(self.population.individuals[index].clone()),
            None => None,
        }
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

    fn crossover(&self, mating_pool: Vec<(usize, usize)>) -> Population {
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

    fn mutation(&self, new_population: Population) -> Population {
        let mutation_chance = self.config.mutation_chance;
        let mutated_population = new_population.individuals.par_iter().map(|individual| {
            let new_individual = individual.chromosome.iter().map(|gene| {
                let mut rng = rand::thread_rng();
                let mutation = rng.gen::<f64>();
                if mutation <= mutation_chance {
                    return gene.mutate();
                }
                return gene.clone();
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

    pub fn run(&mut self) -> (Option<Individual>, Option<f64>) {
        for _ in 1..=self.config.qtd_gen {
            let result = self.evaluate();
            let new_result = self.update_best(result);
            //     if (
            //         self.counter_to_genocide
            //         >= self.problem.config.generations_to_genocide
            //     ):
            //         result = self.genocide()
            //     self.log_generation(generation, result)
            let mating_pool = self.selection(new_result);
            let mut new_population = self.crossover(mating_pool);
            new_population = self.mutation(new_population);
            self.population = new_population;
            // self.log_run_result()
        }
        return (self.best_individual(), self.best_individual_value);
    }
}
