use rand::Rng;

use crate::Individual;

pub trait Crossover {
    #[allow(dead_code)]
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
                let genes_iter = genes_1.iter().zip(genes_2);
                let crossover_point = rng.gen_range(0..genes_1.len());
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
                if crossover_point_1 == genes_1.len() - 1 {
                    return (parent_1.clone(), parent_2.clone());
                }
                let crossover_point_2 =
                    rng.gen_range((crossover_point_1 + 1)..genes_1.len());
                let genes_iter = genes_1.iter().zip(genes_2);
                let (child_genes_1, child_genes_2) = genes_iter
                    .enumerate()
                    .map(|(i, (&gene_1, &gene_2))| {
                        if i >= crossover_point_1 && i < crossover_point_2 {
                            (gene_2, gene_1)
                        } else {
                            (gene_1, gene_2)
                        }
                    })
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
                    Individual::Permuted(child_genes_1),
                    Individual::Permuted(child_genes_2),
                )
            }
        }
    }
}
