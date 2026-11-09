use tracing::{error, info};

use crate::config::ConfigError;

mod cmd;
mod config;
mod logger;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let after_parse_notice = match config::load_config() {
        Err(ConfigError::NoProjectDir) => Some("No project directory found".to_string()),
        Err(ConfigError::ParseEnv(path, err)) if err.not_found() => {
            Some(format!("No config file found at {}", path.display()))
        }
        Err(ConfigError::ParseEnv(config_path, e)) => {
            eprintln!(
                "Failed to parse environment variables in file {}: {}",
                config_path.display(),
                e
            );
            std::process::exit(1);
        }
        Ok(()) => None,
    };

    let conf = config::Config::parse_cli();

    logger::init();

    if let Some(log_level) = conf.global.log_level.as_ref() {
        #[allow(clippy::expect_used)]
        logger::update_log_level(log_level).expect("Failed to update log level");
    }

    if let Some(msg) = after_parse_notice {
        info!("{}", msg);
    }

    if let Err((msg, status)) = conf.cmd.exec().await {
        error!("{}", &msg);
        status.exit_process();
    }
}

impl config::RunCommand {
    pub async fn exec(self) -> cmd::CmdResult {
        match self {
            Self::Server(config) => cmd::server::run(&config).await,
        }
    }
}
