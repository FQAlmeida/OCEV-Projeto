use rand::Rng;
use std::fmt::Display;

#[cfg(features="parallel")]
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub enum IndividualType {
    Binary(Vec<bool>),
    Permuted(Vec<usize>),
}

impl Display for IndividualType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndividualType::Binary(value) => write!(f, "{:?}", *value),
            IndividualType::Permuted(value) => write!(f, "{:?}", *value),
        }
    }
}

impl From<IndividualType> for Vec<bool> {
    fn from(val: IndividualType) -> Self {
        match val {
            IndividualType::Binary(value) => value,
            IndividualType::Permuted(value) => {
                value.iter().map(|&v| v != 0).collect()
            }
        }
    }
}

impl IndividualType {
    #[must_use]
    pub fn mutate(&self, mutation_chance: f64) -> Self {
        match self {
            IndividualType::Binary(genes) => {
                #[cfg(features="parallel")]
                let genes_iter = genes.par_iter();
                #[cfg(not(features="parallel"))]
                let genes_iter = genes.iter();

                IndividualType::Binary(
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
            IndividualType::Permuted(_) => todo!(),
        }
    }
    #[must_use]
    pub fn crossover(
        &self,
        parent_2: &IndividualType,
        crossover_point: usize,
    ) -> (Self, Self) {
        match (self, parent_2) {
            (
                IndividualType::Binary(genes_1),
                IndividualType::Binary(genes_2),
            ) => {
                #[cfg(features="parallel")]
                let genes_iter = genes_1.par_iter().zip(genes_2.par_iter());
                #[cfg(not(features="parallel"))]
                let genes_iter = genes_1.iter().zip(genes_2);

                let (child_genes_1, child_genes_2) = genes_iter
                    .enumerate()
                    .map(|(i, (&gene_1, &gene_2))| {
                        if i < crossover_point {
                            (gene_2, gene_1)
                        } else {
                            (gene_1, gene_2)
                        }
                    })
                    .unzip();
                (
                    IndividualType::Binary(child_genes_1),
                    IndividualType::Binary(child_genes_2),
                )
            }
            (IndividualType::Permuted(_), IndividualType::Permuted(_)) => {
                todo!()
            }
            (IndividualType::Binary(_), IndividualType::Permuted(_)) => todo!(),
            (IndividualType::Permuted(_), IndividualType::Binary(_)) => todo!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Individual {
    pub chromosome: IndividualType,
}

impl Individual {
    #[must_use]
    pub fn new(dim: usize, individual_type: &IndividualType) -> Self {
        let mut rng = rand::thread_rng();
        let chromosome: IndividualType = match individual_type {
            IndividualType::Binary(_) => IndividualType::Binary(
                (0..dim).map(|_| rng.gen::<bool>()).collect(),
            ),
            IndividualType::Permuted(_) => todo!(),
        };
        Individual { chromosome }
    }
    #[must_use]
    pub fn mutate(&self, mutation_chance: f64) -> Self {
        Individual {
            chromosome: self.chromosome.mutate(mutation_chance),
        }
    }
    #[must_use]
    pub fn crossover(
        &self,
        parent_2: &Individual,
        crossover_point: usize,
    ) -> (Self, Self) {
        let (child_1, child_2) = self
            .chromosome
            .crossover(&parent_2.chromosome, crossover_point);
        (
            Individual {
                chromosome: child_1,
            },
            Individual {
                chromosome: child_2,
            },
        )
    }
}

#[derive(Debug, Clone)]
pub struct Population {
    pub individuals: Vec<Individual>,
}

impl Population {
    #[must_use]
    pub fn new(
        qtd_individuals: usize,
        dim: usize,
        individual_type: &IndividualType,
    ) -> Self {
        let individuals: Vec<Individual> = (0..qtd_individuals)
            .map(|_| Individual::new(dim, individual_type))
            .collect();
        Population { individuals }
    }
}
