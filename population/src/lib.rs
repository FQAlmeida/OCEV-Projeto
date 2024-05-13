mod crossover;

use std::fmt::Display;

use rand::{prelude::SliceRandom, Rng};

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

impl IndividualType {
    #[must_use]
    pub fn mutate(&self, mutation_chance: f64) -> Self {
        match self {
            IndividualType::Binary(genes) => {
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
            IndividualType::Permuted(genes) => {
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
                IndividualType::Permuted(new_genes.clone())
            }
        }
    }

    #[must_use]
    pub fn crossover(
        &self,
        parent_2: &IndividualType,
        crossover_point: usize,
    ) -> (Self, Self) {
        match (self, parent_2) {
            (IndividualType::Binary(genes_1), IndividualType::Binary(genes_2)) => {
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
            (
                IndividualType::Permuted(genes_1),
                IndividualType::Permuted(genes_2),
            ) => {
                let mut visited: Vec<usize> = vec![0];
                let mut visited_index: usize = genes_1
                    .iter()
                    .position(|&v| v == genes_2[0])
                    .expect("to find index of value");
                loop {
                    if visited.contains(&visited_index) {
                        break;
                    }
                    visited.push(visited_index);
                    visited_index = genes_1
                        .iter()
                        .position(|&v| v == genes_2[visited_index])
                        .expect("to find index of value");
                }

                let genes_iter = genes_1.iter().zip(genes_2);
                let (child_genes_1, child_genes_2) = genes_iter
                    .enumerate()
                    .map(|(i, (&gene_1, &gene_2))| {
                        if visited.contains(&i) {
                            (gene_1, gene_2)
                        } else {
                            (gene_2, gene_1)
                        }
                    })
                    .unzip();
                (
                    IndividualType::Permuted(child_genes_1),
                    IndividualType::Permuted(child_genes_2),
                )
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
            IndividualType::Binary(_) => {
                IndividualType::Binary((0..dim).map(|_| rng.gen::<bool>()).collect())
            }
            IndividualType::Permuted(_) => {
                let mut genes = (0..dim).collect::<Vec<usize>>();
                genes.shuffle(&mut rng);
                IndividualType::Permuted(genes)
            }
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
