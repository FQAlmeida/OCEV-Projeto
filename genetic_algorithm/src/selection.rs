use rand::Rng;
use random_choice::random_choice;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub trait Selection {
    fn select(&self, result: &[(usize, f64)]) -> Vec<(usize, usize)>;
}

pub struct Tournament {
    kp: f64,
}

impl Tournament {
    pub fn new(kp: f64) -> Self {
        Self { kp }
    }
}

impl Selection for Tournament {
    fn select(&self, result: &[(usize, f64)]) -> Vec<(usize, usize)> {
        let pop_size = result.len();
        let kp = self.kp;
        let mut rng = rand::thread_rng();
        let mut mating_pool: Vec<(usize, usize)> =
            Vec::with_capacity(pop_size / 2);
        for _ in 0..(pop_size / 2) {
            let parent1 = {
                let p1 = rng.gen_range(0..pop_size);
                let p2 = rng.gen_range(0..pop_size);
                let cmp_func = if rng.gen::<f64>() > kp {
                    |a: f64, b: f64| a < b
                } else {
                    |a: f64, b: f64| a > b
                };
                if cmp_func(result[p1].1, result[p2].1) {
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
                if cmp_func(result[p1].1, result[p2].1) {
                    p1
                } else {
                    p2
                }
            };
            mating_pool.push((parent1, parent2));
        }
        mating_pool
    }
}

pub struct RouletteWheel {}

impl RouletteWheel {
    pub fn new() -> Self {
        Self {}
    }
}

impl Selection for RouletteWheel {
    fn select(&self, result: &[(usize, f64)]) -> Vec<(usize, usize)> {
        let pop_size = result.len();
        let result_size = pop_size as f64;
        let mut rng_choice = random_choice();
        let general_probabilities = result
            .par_iter()
            .map(|(_, r)| (*r) / result_size)
            .collect::<Vec<f64>>();
        let pop_index: Vec<usize> =
            result.par_iter().map(|(i, _)| *i).collect();
        let parents_1 = rng_choice.random_choice_f64(
            &pop_index,
            &general_probabilities,
            pop_size / 2,
        );
        let mating_pool: Vec<(usize, usize)> = parents_1
            .par_iter()
            .map(|&parent_1| {
                let mut rng_choice_clone = random_choice();
                let probabilities = result
                    .iter()
                    .filter(|(i, _)| *i != *parent_1)
                    .map(|(_, r)| (*r) / result_size)
                    .collect::<Vec<f64>>();
                let choices: Vec<usize> = pop_index
                    .iter()
                    .filter(|&i| *i != *parent_1)
                    .copied()
                    .collect();
                let parent_2: Vec<usize> = rng_choice_clone
                    .random_choice_f64(&choices, &probabilities, 1)
                    .iter()
                    .map(|&i| *i)
                    .collect();
                (*parent_1, *parent_2.first().expect("parent 2 is empty"))
            })
            .collect();

        mating_pool
    }
}
