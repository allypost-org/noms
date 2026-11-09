use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::Shell;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, path::PathBuf, sync::LazyLock};
use validator::ValidationError;

use directories::ProjectDirs;

pub mod run;

pub static APPLICATION_NAME: &str = "noms";
pub static ORGANIZATION_NAME: &str = "allypost";
pub static ORGANIZATION_QUALIFIER: &str = "net";
pub static CONFIG_FILE_NAME: &str = "noms.env";

pub static CLI_ABOUT: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{}tification {}anagement {}ystem - a way to send notifications to various services in a unified way",
        "NO".bold().underline(),
        "M".bold().underline(),
        "S".bold().underline(),
    )
});

#[derive(Debug, clap::Parser, Serialize, Deserialize)]
#[clap(
    version,
    about = CLI_ABOUT.as_str(),
)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[clap(flatten)]
    pub global: GlobalConfig,

    #[clap(subcommand)]
    pub cmd: run::RunCommand,
}

#[derive(Debug, clap::Args, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalConfig {
    /// The log level to use.
    ///
    /// Can be a comma-separated list of loggers and their levels.
    /// Eg. `noms=trace,noms_cli=debug,request=debug,warn`
    #[clap(long, env = "NOMS_LOG_LEVEL")]
    pub log_level: Option<String>,

    /// Dump shell completions to stdout
    #[arg(
        long,
        default_value = None,
        value_name = "SHELL",
        value_parser = hacky_dump_completions()
    )]
    #[serde(skip)]
    pub dump_completions: Option<Shell>,
}

impl Config {
    pub fn parse_cli() -> Self {
        Self::parse()
    }

    pub fn get_project_dir() -> Option<ProjectDirs> {
        ProjectDirs::from(ORGANIZATION_QUALIFIER, ORGANIZATION_NAME, APPLICATION_NAME)
    }

    pub fn get_config_items() -> Result<HashMap<String, String>, ConfigError> {
        let Some(project_dir) = Self::get_project_dir() else {
            return Err(ConfigError::NoProjectDir);
        };

        let config_path = project_dir.config_dir().join(CONFIG_FILE_NAME);

        let iter = dotenvy::from_path_iter(&config_path)
            .map_err(|e| ConfigError::ParseEnv(config_path.clone(), e))?;

        let mut map = HashMap::new();
        for item in iter {
            let (key, val) = item.map_err(|e| ConfigError::ParseEnv(config_path.clone(), e))?;
            map.insert(key, val);
        }

        Ok(map)
    }
}

pub fn load_config() -> Result<(), ConfigError> {
    load_dotenv()?;
    load_config_env()?;

    Ok(())
}

pub fn load_dotenv() -> Result<(), ConfigError> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(e) if e.not_found() => Ok(()),
        Err(e) => Err(ConfigError::ParseEnv(PathBuf::from(""), e)),
    }
}

pub fn load_config_env() -> Result<(), ConfigError> {
    let map = Config::get_config_items()?;

    for (key, value) in map {
        if env::var(&key).is_err() {
            unsafe {
                env::set_var(key, value);
            }
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("No project directory found")]
    NoProjectDir,
    #[error("Failed to parse environment variables: {0}")]
    ParseEnv(PathBuf, dotenvy::Error),
}

#[must_use]
fn hacky_dump_completions() -> impl clap::builder::TypedValueParser {
    move |s: &str| {
        let parsed = Shell::from_str(s, true);

        if let Ok(shell) = &parsed {
            let bin_name = std::env::current_exe()
                .map_err(|_e| ValidationError::new("Unknown application name"))?
                .file_name()
                .map(|x| x.to_string_lossy().to_string())
                .ok_or_else(|| ValidationError::new("Unknown application name"))?;

            clap_complete::generate(
                *shell,
                &mut Config::command(),
                bin_name,
                &mut std::io::stdout(),
            );
            std::process::exit(0);
        }

        parsed
            .map(|_| ())
            .map_err(|_| ValidationError::new("Invalid shell"))
    }
}
