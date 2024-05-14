use genetic_algorithm::GA;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use loader_config::Config;
use log::info;
use population::Individual;
use problem_factory::problem::Problem;

pub struct Framework {
    problem: Box<dyn Problem + Send + Sync>,
    config: Config,
}

impl Framework {
    #[must_use]
    pub fn new(
        problem: Box<dyn Problem + Send + Sync>,
        config: Config,
    ) -> Framework {
        Framework { problem, config }
    }

    /// # Panics
    /// If I did shit
    #[must_use]
    pub fn run(&self) -> (Option<Individual>, Option<f64>) {
        let mut best_individual: Option<Individual> = None;
        let mut result: Option<f64> = None;
        let m = MultiProgress::new();

        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap();

        let pb =
            m.add(ProgressBar::new(self.config.qtd_runs.try_into().unwrap()));

        pb.set_style(sty);
        pb.set_message("Runs");

        info!("Problem: {}", self.problem.get_name());
        info!("Config: {}", serde_json::to_string(&self.config).unwrap());
        for run in 1..=self.config.qtd_runs {
            info!("Run: {}", run);
            let mut ga = GA::new(&*self.problem, &self.config, &m);
            let (new_individual, new_result) = &ga.run();
            if result.is_none() || new_result.unwrap() > result.unwrap() {
                (best_individual, result) = (
                    Some(new_individual.as_ref().unwrap().clone()),
                    *new_result,
                );
            }
            pb.inc(1);
            info!("End Run: {}", run);
        }
        pb.finish_with_message("All runs completed");
        (best_individual, result)
    }
}
