use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use rand::Rng;

use crate::Individual;

#[async_trait]
pub trait Crossover
where
    Self: Sized,
{
    fn crossover(
        parent_1: &Individual,
        parent_2: &Individual,
    ) -> (Individual, Individual);
}

pub struct OnePointCrossover {}

impl Crossover for OnePointCrossover {
    fn crossover(
        parent_1: &Individual,
        parent_2: &Individual,
    ) -> (Individual, Individual) {
        match (parent_1, parent_2) {
            (Individual::Binary(genes_1), Individual::Binary(genes_2)) => {
                let mut rng = rand::thread_rng();
                let crossover_point = rng.gen_range(0..genes_1.len());
                let genes_iter = genes_1
                    .iter()
                    .take(crossover_point)
                    .zip(genes_2.iter().take(crossover_point));
                let genes_iter_end = genes_1
                    .iter()
                    .skip(crossover_point)
                    .zip(genes_2.iter().skip(crossover_point));
                let (child_genes_1, child_genes_2) = genes_iter
                    .chain(
                        genes_iter_end.map(|(gene_1, gene_2)| (gene_2, gene_1)),
                    )
                    .unzip();
                (
                    Individual::Binary(child_genes_1),
                    Individual::Binary(child_genes_2),
                )
            }
            (Individual::Binary(_), Individual::Permuted(_)) => {
                todo!()
            }
            (Individual::Permuted(_), Individual::Binary(_)) => {
                todo!()
            }
            (Individual::Permuted(_), Individual::Permuted(_)) => {
                todo!()
            }
        }
    }
}

pub struct TwoPointsCrossover {}

impl Crossover for TwoPointsCrossover {
    fn crossover(
        parent_1: &Individual,
        parent_2: &Individual,
    ) -> (Individual, Individual) {
        match (parent_1, parent_2) {
            (Individual::Binary(genes_1), Individual::Binary(genes_2)) => {
                let mut rng = rand::thread_rng();
                let crossover_point_1 = rng.gen_range(0..genes_1.len());
                if crossover_point_1 >= genes_1.len() - 1 {
                    return (parent_1.clone(), parent_2.clone());
                }
                let crossover_point_2 =
                    rng.gen_range((crossover_point_1 + 1)..genes_1.len());
                let genes_iter_start = genes_1
                    .iter()
                    .take(crossover_point_1)
                    .zip(genes_2.iter().take(crossover_point_1));
                let genes_iter_middle = genes_1
                    .iter()
                    .skip(crossover_point_1)
                    .take(crossover_point_2 - crossover_point_1)
                    .zip(
                        genes_2
                            .iter()
                            .skip(crossover_point_1)
                            .take(crossover_point_2 - crossover_point_1),
                    );
                let genes_iter_end = genes_1
                    .iter()
                    .skip(crossover_point_2)
                    .zip(genes_2.iter().skip(crossover_point_2));

                let (child_genes_1, child_genes_2) = genes_iter_start
                    .chain(
                        genes_iter_middle
                            .map(|(gene_1, gene_2)| (gene_2, gene_1)),
                    )
                    .chain(genes_iter_end)
                    .unzip();
                (
                    Individual::Binary(child_genes_1),
                    Individual::Binary(child_genes_2),
                )
            }
            (Individual::Binary(_), Individual::Permuted(_)) => {
                todo!()
            }
            (Individual::Permuted(_), Individual::Binary(_)) => {
                todo!()
            }
            (Individual::Permuted(_), Individual::Permuted(_)) => {
                todo!()
            }
        }
    }
}

pub struct UniformCrossover {}
impl Crossover for UniformCrossover {
    fn crossover(
        parent_1: &Individual,
        parent_2: &Individual,
    ) -> (Individual, Individual) {
        match (parent_1, parent_2) {
            (Individual::Binary(genes_1), Individual::Binary(genes_2)) => {
                let mut rng = rand::thread_rng();
                let genes_iter = genes_1.iter().zip(genes_2);
                let (child_genes_1, child_genes_2) = genes_iter
                    .map(|(&gene_1, &gene_2)| {
                        if rng.gen_bool(0.5) {
                            (gene_1, gene_2)
                        } else {
                            (gene_2, gene_1)
                        }
                    })
                    .unzip();
                (
                    Individual::Binary(child_genes_1),
                    Individual::Binary(child_genes_2),
                )
            }
            (Individual::Binary(_), Individual::Permuted(_)) => todo!(),
            (Individual::Permuted(_), Individual::Binary(_)) => todo!(),
            (Individual::Permuted(_), Individual::Permuted(_)) => todo!(),
        }
    }
}

pub struct CycleCrossover {}

impl Crossover for CycleCrossover {
    fn crossover(
        parent_1: &Individual,
        parent_2: &Individual,
    ) -> (Individual, Individual) {
        match (parent_1, parent_2) {
            (Individual::Binary(_), Individual::Binary(_)) => {
                todo!()
            }
            (Individual::Binary(_), Individual::Permuted(_)) => {
                todo!()
            }
            (Individual::Permuted(_), Individual::Binary(_)) => {
                todo!()
            }
            (Individual::Permuted(genes_1), Individual::Permuted(genes_2)) => {
                let mut visited: HashSet<usize> =
                    HashSet::with_capacity(genes_1.len());
                visited.insert(0);
                let mut visited_index: usize = genes_1
                    .iter()
                    .position(|&v| v == genes_2[0])
                    .expect("to find index of value");
                loop {
                    if visited.contains(&visited_index) {
                        break;
                    }
                    visited.insert(visited_index);
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
                    Individual::Permuted(child_genes_1),
                    Individual::Permuted(child_genes_2),
                )
            }
        }
    }
}

pub struct PartiallyMappedCrossover {}
impl Crossover for PartiallyMappedCrossover {
    fn crossover(
        parent_1: &Individual,
        parent_2: &Individual,
    ) -> (Individual, Individual) {
        match (parent_1, parent_2) {
            (Individual::Binary(_), Individual::Binary(_)) => {
                todo!()
            }
            (Individual::Binary(_), Individual::Permuted(_)) => {
                todo!()
            }
            (Individual::Permuted(_), Individual::Binary(_)) => {
                todo!()
            }
            (Individual::Permuted(genes_1), Individual::Permuted(genes_2)) => {
                let mut rng = rand::thread_rng();
                let crossover_point_1 = rng.gen_range(0..genes_1.len());
                if crossover_point_1 >= genes_1.len() - 1 {
                    return (parent_1.clone(), parent_2.clone());
                }
                let crossover_point_2 =
                    rng.gen_range((crossover_point_1 + 1)..genes_1.len());
                let genes_iter_start = genes_1
                    .iter()
                    .take(crossover_point_1)
                    .zip(genes_2.iter().take(crossover_point_1));
                let genes_iter_middle = genes_1
                    .iter()
                    .skip(crossover_point_1)
                    .take(crossover_point_2 - crossover_point_1)
                    .zip(
                        genes_2
                            .iter()
                            .skip(crossover_point_1)
                            .take(crossover_point_2 - crossover_point_1),
                    );
                let genes_iter_end = genes_1
                    .iter()
                    .skip(crossover_point_2)
                    .zip(genes_2.iter().skip(crossover_point_2));
                let mut hash_map_parent_1: HashMap<usize, usize> =
                    HashMap::with_capacity(
                        crossover_point_2 - crossover_point_1,
                    );
                let mut hash_map_parent_2: HashMap<usize, usize> =
                    HashMap::with_capacity(
                        crossover_point_2 - crossover_point_1,
                    );
                let processed_genes_iter_middle: Vec<(&usize, &usize)> =
                    genes_iter_middle
                        .map(|(gene_1, gene_2)| {
                            hash_map_parent_1.insert(*gene_2, *gene_1);
                            hash_map_parent_2.insert(*gene_1, *gene_2);
                            (gene_2, gene_1)
                        })
                        .collect();
                let (child_genes_1, child_genes_2) = genes_iter_start
                    .map(|(gene_1, gene_2)| {
                        let processed_gene_1 = {
                            let mut aux = gene_1;
                            while hash_map_parent_1.contains_key(aux) {
                                aux = hash_map_parent_1.get(aux).expect(
                                    "Key not found on parent 1 hash map",
                                )
                            }
                            aux
                        };
                        let processed_gene_2 = {
                            let mut aux = gene_2;
                            while hash_map_parent_2.contains_key(aux) {
                                aux = hash_map_parent_2.get(aux).expect(
                                    "Key not found on parent 2 hash map",
                                )
                            }
                            aux
                        };
                        (processed_gene_1, processed_gene_2)
                    })
                    .chain(processed_genes_iter_middle)
                    .chain(genes_iter_end.map(|(gene_1, gene_2)| {
                        let processed_gene_1 = {
                            let mut aux = gene_1;
                            while hash_map_parent_1.contains_key(aux) {
                                aux = hash_map_parent_1.get(aux).expect(
                                    "Key not found on parent 1 hash map",
                                )
                            }
                            aux
                        };
                        let processed_gene_2 = {
                            let mut aux = gene_2;
                            while hash_map_parent_2.contains_key(aux) {
                                aux = hash_map_parent_2.get(aux).expect(
                                    "Key not found on parent 2 hash map",
                                )
                            }
                            aux
                        };
                        (processed_gene_1, processed_gene_2)
                    }))
                    .unzip();
                (
                    Individual::Permuted(child_genes_1),
                    Individual::Permuted(child_genes_2),
                )
            }
        }
    }
}
