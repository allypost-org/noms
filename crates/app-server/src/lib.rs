use std::net::SocketAddr;

use tokio::{net::TcpListener, task::JoinSet};
use tracing::{Instrument, error, info, info_span, trace};

use crate::config::{AdminServerConfig, ApiServerConfig};

mod admin;
mod api;
mod common;
pub mod config;

pub use config::ServerConfig;

pub async fn start(server_config: &ServerConfig) -> Result<(), ServerRunError> {
    info!("Starting noms server");

    let admin_config = server_config.admin.clone();
    let api_config = server_config.api.clone();

    let mut join_set = JoinSet::new();

    join_set.spawn(async move {
        create_admin_server(&admin_config)
            .instrument(info_span!("admin"))
            .await
    });

    join_set.spawn(async move {
        create_api_server(&api_config)
            .instrument(info_span!("api"))
            .await
    });

    while let Some(result) = join_set.join_next().await {
        let result = match result {
            Ok(r) => r,
            Err(e) => {
                error!(?e, "Failed to join server thread. Probably panicked.");
                return Err(e.into());
            }
        };

        match result {
            Ok(()) => {
                info!("Server thread joined");
            }
            Err(e) => {
                error!(?e, "Failed to join server thread. Probably panicked.");
                return Err(e);
            }
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ServerRunError {
    #[error("Failed to parse host {host:?}: {source}")]
    InvalidHost {
        host: String,
        source: std::net::AddrParseError,
    },

    #[error("Failed to join server thread. Probably panicked: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("Failed to bind to address {address:?} for {name} server: {source}")]
    Bind {
        name: &'static str,
        address: SocketAddr,
        source: std::io::Error,
    },

    #[error("Failed to serve {name} server: {source}")]
    Serve {
        name: &'static str,
        source: std::io::Error,
    },
}

async fn create_api_server(config: &ApiServerConfig) -> Result<(), ServerRunError> {
    trace!("Creating API server listener");

    let listener = {
        let address = match config.host_address() {
            Ok(x) => x,
            Err(e) => {
                return Err(ServerRunError::InvalidHost {
                    host: config.host.clone(),
                    source: e,
                });
            }
        };

        TcpListener::bind(address)
            .await
            .map_err(|e| ServerRunError::Bind {
                name: "API",
                address,
                source: e,
            })?
    };

    info!("Started listening for API requests on http://{}", {
        #[allow(clippy::expect_used)]
        {
            listener.local_addr().expect("Failed to get local address")
        }
    });

    trace!("Creating API server app");

    let app = api::routes::create_router(config);

    trace!(?app, "Serving API server");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .map_err(|e| ServerRunError::Serve {
        name: "API",
        source: e,
    })
}

async fn create_admin_server(config: &AdminServerConfig) -> Result<(), ServerRunError> {
    if config.no_run_admin {
        info!("Admin server is disabled");
        return Ok(());
    }

    trace!("Creating admin server listener");

    let listener = {
        let address = match config.address() {
            Ok(x) => x,
            Err(e) => {
                return Err(ServerRunError::InvalidHost {
                    host: config.host.clone(),
                    source: e,
                });
            }
        };

        TcpListener::bind(address)
            .await
            .map_err(|e| ServerRunError::Bind {
                name: "Admin",
                address,
                source: e,
            })?
    };

    info!("Started listening for admin requests on http://{}", {
        #[allow(clippy::expect_used)]
        {
            listener.local_addr().expect("Failed to get local address")
        }
    });

    trace!("Creating admin server app");

    let app = admin::routes::create_router();

    trace!(?app, "Serving admin server");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .map_err(|e| ServerRunError::Serve {
        name: "Admin",
        source: e,
    })
}
