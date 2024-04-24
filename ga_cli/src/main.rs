use anyhow::Result;
use genetic_framework::Framework;
use inquire::{list_option::ListOption, InquireError, Select};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
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

fn config_tracing(problem_name: &str) -> Result<()> {
    let file_path = format!(
        "data/outputs/{}-{}.log",
        problem_name,
        chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
    );

    // Logging to log file.
    let log_file = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("log_file", Box::new(log_file)))
        .build(
            Root::builder()
                .appender("log_file")
                .build(LevelFilter::Info),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let options: Vec<&str> = vec!["SAT-3", "RADIO", "ALGEBRAIC-FUNCTION"];
    let problem_name_answer: Result<&str, InquireError> =
        Select::new("Which problem to run?", options).prompt();
    let problem_name = match problem_name_answer {
        Ok(problem_name) => problem_name,
        Err(_) => panic!("Problem not found"),
    };
    config_tracing(problem_name)?;

    let instances_options: Vec<String> =
        fs::read_dir(format!("data/instances/{}", problem_name.to_lowercase()))
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
            entry.path().extension().unwrap() == "json"
                && entry
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
