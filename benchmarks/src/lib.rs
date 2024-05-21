use anyhow::Result;
use genetic_framework::Framework;
use loader_config::Config;
use problem_factory::{
    algebraic_function::{self, AlgebraicFunction},
    nqueens::{self, NQueens},
    radio::{self, Radio},
    sat_3::{self, SAT3},
};

pub fn run_sat(instance: &str, config: Config) -> Result<()> {
    let problem =
        sat_3::load_instance(instance).expect("Unable to load instance");
    let problem = Box::new(SAT3::new(&problem, config));

    let ga_framework = Framework::new(problem, config);
    println!("{:?}", ga_framework.run());

    Ok(())
}

pub fn run_algebraic(instance: &str, config: Config) -> Result<()> {
    let problem = algebraic_function::load_instance(instance)
        .expect("Unable to load instance");
    let problem = Box::new(AlgebraicFunction::new(problem, config));

    let ga_framework = Framework::new(problem, config);
    println!("{:?}", ga_framework.run());

    Ok(())
}

pub fn run_radio(instance: &str, config: Config) -> Result<()> {
    let problem =
        radio::load_instance(instance).expect("Unable to load instance");
    let problem = Box::new(Radio::new(problem, config));

    let ga_framework = Framework::new(problem, config);
    println!("{:?}", ga_framework.run());

    Ok(())
}

pub fn run_nqueens(instance: &str, config: Config) -> Result<()> {
    let problem =
        nqueens::load_instance(instance).expect("Unable to load instance");
    let problem = Box::new(NQueens::new(problem, config));

    let ga_framework = Framework::new(problem, config);
    println!("{:?}", ga_framework.run());

    Ok(())
}
