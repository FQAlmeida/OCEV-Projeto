use anyhow::Result;
use genetic_framework::Framework;
use inquire::{list_option::ListOption, InquireError, Select};
use std::fs::{self};

fn format_path(path: ListOption<&String>) -> String {
    fs::canonicalize(path.value)
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn config_tracing() -> Result<()> {
    let file_appender = tracing_appender::rolling::never("data/outputs/", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt().with_writer(non_blocking).init();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    config_tracing()?;

    let options: Vec<&str> = vec!["SAT-3"];
    let problem_name_answer: Result<&str, InquireError> =
        Select::new("Which problem to run?", options).prompt();
    let problem_name = match problem_name_answer {
        Ok(problem_name) => problem_name,
        Err(_) => panic!("Problem not found"),
    };

    let instances_options: Vec<String> = fs::read_dir(format!("data/instances/{}", problem_name))
        .unwrap()
        .map(|entry| {
            fs::canonicalize(entry.unwrap().path())
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
        })
        .collect();
    let instance_answer: Result<String, InquireError> =
        Select::new("Which instance to run?", instances_options)
            .with_formatter(&format_path)
            .prompt();
    let instance = match instance_answer {
        Ok(instance) => instance,
        Err(_) => panic!("Instance not found"),
    };

    let config_options: Vec<String> = fs::read_dir("data/config")
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| {
            entry
                .file_name()
                .into_string()
                .unwrap()
                .starts_with(problem_name.to_lowercase().as_str())
        })
        .map(|entry| {
            fs::canonicalize(entry.path())
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
        })
        .collect();
    let config_answer: Result<String, InquireError> =
        Select::new("Which config to run?", config_options)
            .with_formatter(&format_path)
            .prompt();
    let config_path = match config_answer {
        Ok(config) => config,
        Err(_) => panic!("Config not found"),
    };

    let (problem, config) = problem_factory::problem_factory(problem_name, &instance, &config_path);
    let ga_framework = Framework::new(problem, config);
    println!("{:?}", ga_framework.run());

    Ok(())
}
