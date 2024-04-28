use std::path::Path;

use loader_config::Config;
use problem::Problem;

pub mod sat_3;
use sat_3::SAT3;
pub mod algebraic_function;
pub mod radio;
use algebraic_function::AlgebraicFunction;
use radio::Radio;

/// # Panics
/// Panics if the configuration file is not found
/// or the problem is not implemented
pub fn problem_factory<P>(
    problem: &str,
    instance: &str,
    config_path: P,
) -> (Box<dyn Problem + Send + Sync>, Config)
where
    P: AsRef<Path>,
{
    let config = Config::new(config_path).unwrap();
    match problem.to_uppercase().as_str() {
        "SAT-3" => {
            let problem = sat_3::load_instance(instance).unwrap();
            (Box::new(SAT3::new(&problem, config)), config)
        }
        "RADIO" => {
            let problem = radio::load_instance(instance).unwrap();
            (Box::new(Radio::new(problem, config)), config)
        }
        "ALGEBRAIC-FUNCTION" => {
            let problem = algebraic_function::load_instance(instance).unwrap();
            (Box::new(AlgebraicFunction::new(problem, config)), config)
        }
        _ => panic!("Problem not found"),
    }
}
