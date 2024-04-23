use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use individual_creation::{Individual, IndividualType};
use loader_config::Config;
use problem::Problem;
use rayon::prelude::*;

pub struct SAT3 {
    config: Config,
    clause_id: Vec<(i32, i32, i32)>,
    clause_neg: Vec<(bool, bool, bool)>,
}

impl SAT3 {
    pub fn new(problem: Vec<(i32, i32, i32)>, config: Config) -> SAT3 {
        let (clause_id, clause_neg) = SAT3::clauses(&problem);
        return SAT3 {
            config,
            clause_id,
            clause_neg,
        };
    }
}

impl Problem for SAT3 {
    fn decode(&self, individual: &Individual) -> Vec<f64> {
        return individual
            .chromosome
            .iter()
            .map(|i| match i {
                IndividualType::Binary(value) => {
                    if *value {
                        return 1.0;
                    } else {
                        return 0.0;
                    }
                }
            })
            .collect();
    }

    fn get_config(&self) -> &Config {
        return &self.config;
    }

    fn normed_objective(&self, individual: &Vec<f64>) -> f64 {
        return self.objective(individual);
    }

    fn constraint(&self, _: &Vec<f64>) -> f64 {
        return 0.0;
    }

    fn fitness(&self, individual: &Individual) -> f64 {
        let config = self.get_config();
        let decoded_individual = self.decode(individual);
        let obj = self.normed_objective(&decoded_individual);
        let constraint = self.constraint(&decoded_individual);
        return obj + config.constraint_penalty * constraint;
    }

    fn objective(&self, individual: &Vec<f64>) -> f64 {
        let clauses_satisfied: f64 = self
            .clause_id
            .par_iter()
            .zip(self.clause_neg.par_iter())
            .map(|i| {
                let (clause, clause_neg) = i;
                let solution = individual;
                let evaluated_solution = self.eval_solution(solution, clause, clause_neg);
                return evaluated_solution as u32 as f64;
            })
            .sum::<f64>()
            .into();
        return clauses_satisfied;
    }
}

impl SAT3 {
    fn clauses(problem: &Vec<(i32, i32, i32)>) -> (Vec<(i32, i32, i32)>, Vec<(bool, bool, bool)>) {
        let clause_id = problem
            .iter()
            .map(|(a, b, c)| (a.abs() - 1, b.abs() - 1, c.abs() - 1))
            .collect();
        let clause_neg = problem
            .iter()
            .map(|(a, b, c)| (*a < 0, *b < 0, *c < 0))
            .collect();
        return (clause_id, clause_neg);
    }

    fn eval_solution(
        &self,
        solution: &Vec<f64>,
        clause_id: &(i32, i32, i32),
        clause_neg: &(bool, bool, bool),
    ) -> bool {
        let (a, b, c) = clause_id;
        let (na, nb, nc) = clause_neg;

        let solution_a: bool = solution[*a as usize] == 1.0;
        let solution_b: bool = solution[*b as usize] == 1.0;
        let solution_c: bool = solution[*c as usize] == 1.0;
        let checked_solution_a = (!solution_a && *na) || (solution_a && !*na);
        let checked_solution_b = (!solution_b && *nb) || (solution_b && !*nb);
        let checked_solution_c = (!solution_c && *nc) || (solution_c && !*nc);
        return checked_solution_a || checked_solution_b || checked_solution_c;
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_instance<P>(filename: P) -> io::Result<Vec<(i32, i32, i32)>>
where
    P: AsRef<Path>,
{
    let problem = read_lines(filename)?
        .map(|line| {
            let line = line.unwrap();
            let mut clause = line.split_whitespace();
            let a: i32 = clause.next().unwrap().parse().unwrap();
            let b: i32 = clause.next().unwrap().parse().unwrap();
            let c: i32 = clause.next().unwrap().parse().unwrap();
            return (a, b, c);
        })
        .collect::<Vec<(i32, i32, i32)>>();
    Ok(problem)
}
