use genetic_algorithm::GA;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
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
        let m = MultiProgress::new();

        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap();
    
        let pb = m.add(ProgressBar::new(self.config.qtd_runs.try_into().unwrap()));

        pb.set_style(sty);
        pb.set_message("Runs");
        
        for _ in 1..=self.config.qtd_runs {
            let mut ga = GA::new(&self.problem, &self.config, &m);
            let (new_individual, new_result) = &ga.run();
            if result.is_none() || new_result.unwrap() > result.unwrap() {
                (best_individual, result) =
                    (Some(new_individual.unwrap().clone()), new_result.clone());
            }
            pb.inc(1)
        }
        pb.finish_with_message("All runs completed");
        return (best_individual, result);
    }
}
