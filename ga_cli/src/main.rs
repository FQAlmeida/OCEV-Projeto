use clap::Parser;
use genetic_framework::Framework;
use utils::Problems;

use crate::logger::config_tracing;

mod logger;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the problem
    #[arg(short, long, value_enum)]
    problem_name: Option<Problems>,

    /// Path to the instance file
    #[arg(short, long)]
    instance: Option<String>,

    /// Path to the config file
    #[arg(short, long)]
    config: Option<String>,
}

fn validate_args(args: &Args) {
    if args.problem_name.is_none() {
        if args.instance.is_some() || args.config.is_some() {
            panic!("Problem name is required if instance or config is provided");
        }
        return;
    }
    let problem_name = args
        .problem_name
        .as_ref()
        .expect("Unable to unwrap problem name").to_string();
    if args.instance.is_some() {
        let instance = args.instance.as_ref().expect("Unable to unwrap instance");
        utils::validate_instance(&problem_name, instance)
            .expect("Instance is invalid");
    }
    if args.config.is_some() {
        let config = args.config.as_ref().expect("Unable to unwrap config");
        utils::validate_config(&problem_name, config)
            .expect("Config is invalid");
    }
}

fn main() {
    let args = Args::parse();
    validate_args(&args);

    let problem_name = match args.problem_name {
        Some(problem_name) => problem_name.to_string(),
        None => utils::ask_for_problem_name().expect("Problem not found"),
    };
    config_tracing(&problem_name);

    let instance = match args.instance {
        Some(instance) => instance,
        None => utils::ask_for_instance(&problem_name).expect("Instance not found"),
    };

    let config_path = match args.config {
        Some(config_path) => config_path,
        None => utils::ask_for_config(&problem_name).expect("Config not found"),
    };

    let (problem, config) =
        problem_factory::problem_factory(&problem_name, &instance, &config_path);
    let ga_framework = Framework::new(problem, config);
    println!("{:?}", ga_framework.run());
}
