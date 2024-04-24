use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use individual_creation::{Individual, IndividualType};
use loader_config::Config;
use problem::Problem;

pub struct ProblemAlgebraicFunction {
    max_y: f64,
    max_x: f64,
    min_y: f64,
    min_x: f64,
}

pub struct AlgebraicFunction {
    config: Config,
    problem: ProblemAlgebraicFunction,
}

impl AlgebraicFunction {
    pub fn new(problem: ProblemAlgebraicFunction, config: Config) -> Self {
        AlgebraicFunction { config, problem }
    }
}

impl Problem for AlgebraicFunction {
    fn decode(&self, individual: &Individual) -> Vec<f64> {
        let decimal = individual
            .chromosome
            .iter()
            .map(|i| match i {
                IndividualType::Binary(value) => f64::from(u32::from(*value)),
                IndividualType::Permuted(_) => todo!(),
            })
            .fold(0.0, |a, b| 2.0 * a + b);
        let value = self.problem.min_x
            + ((self.problem.max_x - self.problem.min_x)
                * (decimal
                    / (2.0_f64.powf(individual.chromosome.len() as f64)
                        - 1.0)));
        vec![value]
    }

    fn get_config(&self) -> &Config {
        &self.config
    }

    fn normed_objective(&self, individual: &[f64]) -> f64 {
        (self.objective(individual) - self.problem.min_y)
            / (self.problem.max_y - self.problem.min_y)
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
        let x = individual[0];

        f64::cos(20.0 * x) - (x.abs() / 2.0) + (x.powf(3.0) / 4.0)
    }

    fn get_name(&self) -> String {
        String::from("ALGEBRAIC-FUNCTION")
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_instance<P>(filename: P) -> io::Result<ProblemAlgebraicFunction>
where
    P: AsRef<Path>,
{
    let problem = read_lines(filename)?
        .map(|line| {
            let line = line.unwrap();
            return line
                .split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect();
        })
        .collect::<Vec<Vec<f64>>>();
    Ok(ProblemAlgebraicFunction {
        min_x: problem[0][0],
        max_x: problem[0][1],
        min_y: problem[1][0],
        max_y: problem[1][1],
    })
}
