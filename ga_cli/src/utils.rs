use std::fs;

use anyhow::Result;
use clap::ValueEnum;
use inquire::{list_option::ListOption, Select};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

pub fn format_path(path: ListOption<&String>) -> String {
    fs::canonicalize(path.value)
        .expect("Failed to canonicalize path")
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to convert to string")
        .to_string()
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, EnumIter, Display,
)]
pub enum Problems {
    #[strum(serialize = "SAT-3")]
    SAT3,
    #[strum(serialize = "RADIO")]
    Radio,
    #[strum(serialize = "ALGEBRAIC-FUNCTION")]
    AlgebraicFunction,
    #[strum(serialize = "NQUEENS")]
    NQueens,
}

pub fn ask_for_problem_name() -> Result<String> {
    let options: Vec<String> = Problems::iter().map(|p| p.to_string()).collect();
    Ok(Select::new("Which problem to run?", options)
        .prompt()?
        .to_owned())
}

pub fn validate_instance(problem_name: &str, instance: &str) -> Result<()> {
    if !instance.contains(&problem_name.to_lowercase()) {
        return Err(anyhow::anyhow!("Invalid instance path"));
    }
    if !fs::metadata(instance)?.is_file() {
        return Err(anyhow::anyhow!("Instance not found"));
    }
    Ok(())
}

pub fn ask_for_instance(problem_name: &str) -> Result<String> {
    let instances_options: Vec<String> =
        fs::read_dir(format!("data/instances/{}", problem_name.to_lowercase()))
            .expect("Unable to find instances")
            .map(|entry| {
                fs::canonicalize(entry.expect("Unable to retrieve entry").path())
                    .expect("Unable to canonicalize path")
                    .into_os_string()
                    .into_string()
                    .expect("Unable to convert to string")
            })
            .collect();
    Ok(Select::new("Which instance to run?", instances_options)
        .with_formatter(&format_path)
        .prompt()?)
}

pub fn validate_config(problem_name: &str, config: &str) -> Result<()> {
    if !config.contains(&problem_name.to_lowercase()) {
        return Err(anyhow::anyhow!("Invalid config path"));
    }
    if !fs::metadata(config)?.is_file() {
        return Err(anyhow::anyhow!("Config not found"));
    }
    Ok(())
}

pub fn ask_for_config(problem_name: &str) -> Result<String> {
    let config_options: Vec<String> = fs::read_dir("data/config")
        .expect("Unable to find config files")
        .map(std::result::Result::unwrap)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .expect("Unable to retrieve file extension")
                == "json"
                && entry
                    .file_name()
                    .into_string()
                    .expect("Unable to convert to string")
                    .starts_with(problem_name.to_lowercase().as_str())
        })
        .map(|entry| {
            fs::canonicalize(entry.path())
                .expect("Unable to canonicalize path")
                .into_os_string()
                .into_string()
                .expect("Unable to convert to string")
        })
        .collect();
    Ok(Select::new("Which config to run?", config_options)
        .with_formatter(&format_path)
        .prompt()?)
}
