use logger::{p_err, p_notice};

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
            p_err(&format!(
                "Failed to parse environment variables in file {}: {}",
                config_path.display(),
                e
            ));
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
        p_notice(&msg);
    }

    if let Err((msg, status)) = conf.cmd.exec().await {
        p_err(&msg);
        status.exit_process();
    }
}

impl config::run::RunCommand {
    pub async fn exec(self) -> cmd::CmdResult {
        match self {
            Self::Send(send) => cmd::send::run(send).await,
            #[cfg(feature = "server")]
            Self::Server(server) => cmd::server::run(&server).await,
        }
    }
}
