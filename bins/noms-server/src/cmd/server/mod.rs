use app_server::ServerConfig;

use crate::cmd::{CmdResult, ExitCode};

pub async fn run(config: &ServerConfig) -> CmdResult {
    if let Err(e) = app_server::start(config).await {
        return Err((e.to_string(), ExitCode::from(1)));
    }

    Ok(())
}
