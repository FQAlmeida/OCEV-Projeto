use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use individual_creation::{Individual, IndividualType};
use loader_config::Config;
use problem::Problem;

pub struct SAT3 {
    config: Config,
    clause_id: Vec<(u32, u32, u32)>,
    clause_neg: Vec<(bool, bool, bool)>,
}

impl SAT3 {
    pub fn new(problem: &[(i32, i32, i32)], config: Config) -> SAT3 {
        let (clause_id, clause_neg) = SAT3::clauses(problem);
        SAT3 {
            config,
            clause_id,
            clause_neg,
        }
    }
}

impl Problem for SAT3 {
    fn decode(&self, individual: &Individual) -> Vec<f64> {
        match &individual.chromosome {
            IndividualType::Binary(value) => {
                value.iter().map(|&v| f64::from(u32::from(v)))
            }
            IndividualType::Permuted(_) => todo!(),
        }
        .collect()
    }

    fn get_config(&self) -> &Config {
        &self.config
    }

    fn normed_objective(&self, individual: &[f64]) -> f64 {
        self.objective(individual)
    }

    fn constraint(&self, _: &[f64]) -> f64 {
        0.0
    }

    fn fitness(&self, individual: &Individual) -> f64 {
        let config = self.get_config();
        let decoded_individual = self.decode(individual);
        let obj = self.normed_objective(&decoded_individual);
        let constraint = self.constraint(&decoded_individual);
        obj + config.constraint_penalty * constraint
    }

    fn objective(&self, individual: &[f64]) -> f64 {
        let clauses_neg_iter = self.clause_neg.iter();
        let clauses_satisfied_iter = self.clause_id.iter();

        clauses_satisfied_iter
            .zip(clauses_neg_iter)
            .map(|i| {
                let (clause, clause_neg) = i;
                let solution = individual;
                let evaluated_solution =
                    SAT3::eval_solution(solution, clause, *clause_neg);
                f64::from(u32::from(evaluated_solution))
            })
            .sum::<f64>()
    }

    fn get_name(&self) -> String {
        String::from("SAT-3")
    }
}
type ClausesType = (Vec<(u32, u32, u32)>, Vec<(bool, bool, bool)>);
impl SAT3 {
    fn clauses(problem: &[(i32, i32, i32)]) -> ClausesType {
        let clause_id = problem
            .iter()
            .map(|(a, b, c)| {
                (
                    (a.abs() - 1) as u32,
                    (b.abs() - 1) as u32,
                    (c.abs() - 1) as u32,
                )
            })
            .collect();
        let clause_neg = problem
            .iter()
            .map(|(a, b, c)| (*a < 0, *b < 0, *c < 0))
            .collect();
        (clause_id, clause_neg)
    }

    fn eval_solution(
        solution: &[f64],
        clause_id: &(u32, u32, u32),
        clause_neg: (bool, bool, bool),
    ) -> bool {
        let (a, b, c) = clause_id;
        let (na, nb, nc) = clause_neg;

        let solution_a: bool =
            (solution[*a as usize] - 1.0_f64).abs() < f64::EPSILON;
        let solution_b: bool =
            (solution[*b as usize] - 1.0_f64).abs() < f64::EPSILON;
        let solution_c: bool =
            (solution[*c as usize] - 1.0_f64).abs() < f64::EPSILON;
        let checked_solution_a = (!solution_a && na) || (solution_a && !na);
        let checked_solution_b = (!solution_b && nb) || (solution_b && !nb);
        let checked_solution_c = (!solution_c && nc) || (solution_c && !nc);
        checked_solution_a || checked_solution_b || checked_solution_c
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
            (a, b, c)
        })
        .collect::<Vec<(i32, i32, i32)>>();
    Ok(problem)
}
