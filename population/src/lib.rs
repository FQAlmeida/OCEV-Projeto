mod crossover;

use std::fmt::Display;

use crossover::{
    Crossover, CycleCrossover, OnePointCrossover, TwoPointsCrossover,
};
use loader_config::{CrossoverMethod, PopType};
use rand::{prelude::SliceRandom, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone, Debug)]
pub enum Individual {
    Binary(Vec<bool>),
    Permuted(Vec<usize>),
}

impl Display for Individual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Individual::Binary(value) => write!(f, "{:?}", *value),
            Individual::Permuted(value) => write!(f, "{:?}", *value),
        }
    }
}

impl Individual {
    #[must_use]
    pub fn mutate(&self, mutation_chance: f64) -> Self {
        match self {
            Individual::Binary(genes) => {
                let genes_iter = genes.iter();

                Individual::Binary(
                    genes_iter
                        .map(|gene| {
                            let mut rng = rand::thread_rng();
                            let mutation = rng.gen::<f64>();
                            if mutation <= mutation_chance {
                                return !gene;
                            }
                            *gene
                        })
                        .collect(),
                )
            }
            Individual::Permuted(genes) => {
                let mut new_genes = genes.clone();
                for i in 0..genes.len() {
                    let mut rng = rand::thread_rng();
                    let mutation = rng.gen::<f64>();
                    if mutation <= mutation_chance {
                        let new_gene = rng.gen_range(0..genes.len());
                        (new_genes[i], new_genes[new_gene]) =
                            (new_genes[new_gene], new_genes[i]);
                    }
                }
                Individual::Permuted(new_genes.clone())
            }
        }
    }

    #[must_use]
    pub fn crossover(
        &self,
        parent_2: &Individual,
        crossover_method: &CrossoverMethod,
    ) -> (Self, Self) {
        match crossover_method {
            CrossoverMethod::OnePoint => {
                OnePointCrossover::crossover(self, parent_2)
            }
            CrossoverMethod::TwoPoints => {
                TwoPointsCrossover::crossover(self, parent_2)
            }
            CrossoverMethod::Uniform => todo!(),
            CrossoverMethod::Cycle => CycleCrossover::crossover(self, parent_2),
            CrossoverMethod::Permuted => todo!(),
        }
    }

    #[must_use]
    pub fn new(dim: usize, individual_type: &PopType) -> Self {
        let mut rng = rand::thread_rng();
        let chromosome: Individual = match individual_type {
            PopType::Binary => Individual::Binary(
                (0..dim).map(|_| rng.gen::<bool>()).collect(),
            ),
            PopType::Permuted => {
                let mut genes = (0..dim).collect::<Vec<usize>>();
                genes.shuffle(&mut rng);
                Individual::Permuted(genes)
            }
            PopType::Real => todo!(),
            PopType::Integer => todo!(),
        };
        chromosome
    }
}

#[derive(Debug, Clone)]
pub struct Population {
    pub individuals: Vec<Individual>,
}

impl Population {
    #[must_use]
    pub fn new(qtd_individuals: usize, dim: usize, pop_type: &PopType) -> Self {
        let individuals: Vec<Individual> = (0..qtd_individuals)
            .into_par_iter()
            .map(|_| Individual::new(dim, individual_type))
            .collect();
        Population { individuals }
    }
}
