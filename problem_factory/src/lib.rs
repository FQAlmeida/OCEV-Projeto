use loader_config::Config;
use problem::Problem;
use sat_3::{load_instance, SAT3};

pub fn problem_factory(
    problem: &str,
    instance: &str,
    config_path: &str,
) -> (Box<dyn Problem + Send + Sync>, Config) {
    match problem.to_uppercase().as_str() {
        "SAT-3" => {
            let config = Config::load(config_path).unwrap();
            let problem = load_instance(instance).unwrap();
            return (Box::new(SAT3::new(problem, config)), config);
        }
        _ => panic!("Problem not found"),
    }
}
