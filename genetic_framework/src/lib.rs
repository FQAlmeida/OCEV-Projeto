use genetic_algorithm::GA;
use individual_creation::Individual;
use loader_config::Config;
use problem::Problem;

pub struct Framework {
    problem: Box<dyn Problem + Send + Sync>,
    config: Config,
}

impl Framework {
    pub fn new(problem: Box<dyn Problem + Send + Sync>, config: Config) -> Framework {
        Framework { problem, config }
    }

    pub fn run(&self) -> (Option<Individual>, Option<f64>) {
        let mut best_individual: Option<Individual> = None;
        let mut result: Option<f64> = None;
        for _ in 0..self.config.qtd_runs {
            let mut ga = GA::new(&self.problem, &self.config);
            let (new_individual, new_result) = ga.run();
            if result.is_none() || new_result.unwrap() > result.unwrap() {
                (best_individual, result) = (new_individual, new_result);
            }
        }
        return (best_individual, result);
    }
}
