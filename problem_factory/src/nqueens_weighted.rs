use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use loader_config::Config;
use population::Individual;

use crate::Problem;

// #[cfg(not(feature = "sequential"))]
// use rayon::prelude::*;

pub struct ProblemNQueensWeighted {
    pub board_size: usize,
    pub max_fo: f64,
    pub max_h: f64,
}

pub struct NQueensWeighted {
    config: Config,
    problem: ProblemNQueensWeighted,
}

impl NQueensWeighted {
    pub fn new(problem: ProblemNQueensWeighted, config: Config) -> Self {
        NQueensWeighted { problem, config }
    }
}

impl Problem for NQueensWeighted {
    fn decode(&self, individual: &Individual) -> Vec<f64> {
        match &individual {
            Individual::Binary(_) => {
                todo!()
            }
            Individual::Permuted(value) => value.iter().map(|&v| v as f64),
        }
        .collect()
    }

    fn get_config(&self) -> &Config {
        &self.config
    }

    fn normed_objective(&self, individual: &[f64]) -> f64 {
        self.objective(individual) / self.problem.max_fo
    }

    fn constraint(&self, individual: &[f64]) -> f64 {
        let collisions: usize = individual[0..individual.len() - 1]
            .iter()
            .enumerate()
            .map(|(line, &queen)| {
                let queen_col = queen as usize;
                for (next_line, &next_queen) in
                    individual[line + 1..].iter().enumerate()
                {
                    let offset = next_line + 1;
                    if queen_col + offset >= self.problem.board_size
                        && queen_col < offset
                    {
                        return 0;
                    }
                    let next_queen_col = next_queen as usize;
                    if (queen_col >= offset
                        && next_queen_col == queen_col - offset)
                        || (queen_col + offset < self.problem.board_size
                            && next_queen_col == queen_col + offset)
                    {
                        return 1;
                    }
                }
                0
            })
            .sum();
        (collisions as f64) / self.problem.max_h
    }

    fn fitness(&self, individual: &Individual) -> f64 {
        let config = self.get_config();
        let decoded_individual = self.decode(individual);
        let obj = self.normed_objective(&decoded_individual);
        let constraint = self.constraint(&decoded_individual);
        obj + config.constraint_penalty * constraint
    }

    fn objective(&self, individual: &[f64]) -> f64 {
        let board_size: f64 = self.problem.board_size as f64;
        let roots = individual
            .iter()
            .enumerate()
            .step_by(2)
            .fold(0.0, |acc, (line, &col)| {
                acc + ((line as f64) * board_size + col).sqrt()
            });
        let logs = individual.iter().enumerate().skip(1).step_by(2).fold(
            0.0,
            |acc, (line, &col)| {
                acc + ((line as f64) * board_size + col).log10()
            },
        );
        return roots + logs;
    }

    fn get_name(&self) -> String {
        String::from("N-QUEENS-WEIGHTED")
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_instance<P>(filename: P) -> io::Result<ProblemNQueensWeighted>
where
    P: AsRef<Path>,
{
    let problem = read_lines(filename)?
        .map(|line| {
            let line = line.expect("Unable to read line");
            return line
                .split_whitespace()
                .map(|i| i.parse().expect("Unable to parse number"))
                .collect();
        })
        .collect::<Vec<Vec<f64>>>();
    let board_size = problem[0][0];
    let max_fo = (0..board_size as usize)
        .map(|i| {
            if i % 2 == 0 {
                (i as f64 * board_size).sqrt()
            } else {
                (i as f64 * board_size).log10()
            }
        })
        .sum::<f64>();
    Ok(ProblemNQueensWeighted {
        board_size: board_size as usize,
        max_h: board_size - 1.0,
        max_fo,
    })
}
