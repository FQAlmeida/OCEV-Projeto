use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use loader_config::Config;
use population::Individual;

use crate::Problem;

pub struct ProblemRadio {
    max_h: f64,
    max_fo: f64,
    qtd_employees: usize,
    profit_a: f64,
    profit_b: f64,
}

pub struct Radio {
    config: Config,
    problem: ProblemRadio,
}

impl Radio {
    pub fn new(problem: ProblemRadio, config: Config) -> Self {
        Radio { config, problem }
    }
}

impl Problem for Radio {
    fn decode(&self, individual: &Individual) -> Vec<f64> {
        let values: Vec<f64> = match &individual {
            Individual::Binary(value) => {
                value.iter().map(|&v| f64::from(u32::from(v)))
            }
            Individual::Permuted(_) => todo!(),
        }
        .collect();
        let qtd_line_a: f64 = values[0..5]
            .to_owned()
            .clone()
            .iter()
            .fold(0.0, |a, &b| 2.0 * a + b);
        let qtd_line_b: f64 = values[5..]
            .to_owned()
            .clone()
            .iter()
            .fold(0.0, |a, &b| 2.0 * a + b);
        vec![qtd_line_a, qtd_line_b]
    }

    fn get_config(&self) -> &Config {
        &self.config
    }

    fn normed_objective(&self, individual: &[f64]) -> f64 {
        self.objective(individual) / self.problem.max_fo
    }

    fn constraint(&self, individual: &[f64]) -> f64 {
        let qtd_1 = individual[0];
        let qtd_2 = individual[1];
        let penalty = ((qtd_1 + (2.0 * qtd_2))
            - self.problem.qtd_employees as f64)
            / self.problem.max_h;

        let penalty_2 = f64::max(0.0, (qtd_1 - 24.0) / 8.0);
        let penalty_3 = f64::max(0.0, (qtd_2 - 16.0) / 16.0);
        let p = (penalty + penalty_2 + penalty_3) / 3.0;
        f64::max(0.0, p)
    }

    fn fitness(&self, individual: &Individual) -> f64 {
        let config = self.get_config();
        let decoded_individual = self.decode(individual);
        let obj = self.normed_objective(&decoded_individual);
        let constraint = self.constraint(&decoded_individual);
        obj + config.constraint_penalty * constraint
    }

    fn objective(&self, individual: &[f64]) -> f64 {
        let qtd_1 = individual[0];
        let qtd_2 = individual[1];

        self.problem.profit_a * qtd_1 + self.problem.profit_b * qtd_2
    }

    fn get_name(&self) -> String {
        String::from("RADIO")
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_instance<P>(filename: P) -> io::Result<ProblemRadio>
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
    Ok(ProblemRadio {
        profit_a: problem[0][0],
        profit_b: problem[0][1],
        qtd_employees: problem[1][0] as usize,
        max_fo: problem[2][0],
        max_h: problem[3][0],
    })
}
