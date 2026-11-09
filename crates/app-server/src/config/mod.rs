use std::net::AddrParseError;
use std::path::PathBuf;
use std::{fmt::Write, net::SocketAddr};

use serde::{Deserialize, Serialize};

#[derive(Debug, clap::Args, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    /// The API server configuration.
    #[clap(flatten)]
    pub api: ApiServerConfig,

    /// The admin server configuration.
    ///
    /// If no options are set, the admin server will not be started.
    #[clap(flatten)]
    pub admin: AdminServerConfig,

    /// The libsql database URL to use.
    ///
    /// Should be a valid database URL, such as `sqlite:./db.sqlite`.
    #[clap(short = 'D', long, default_value = ":memory:", env = "NOMS_DATABASE_URL", value_parser = DatabaseUrlParser, hide_env_values = true)]
    pub database_url: DatabaseUrl,
}

#[derive(Clone, clap::Args, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ApiServerConfig {
    /// The port the API server listens on.
    #[clap(
        short = 'P',
        long,
        default_value = "9009",
        env = "NOMS_SERVER_API_PORT"
    )]
    pub port: u16,

    /// The host to bind the API server to.
    ///
    /// Should usually be either `0.0.0.0` if you want to bind to all interfaces aka the public,
    /// or `127.0.0.1` if you don't want to expose the server to the outside world.
    #[clap(
        short = 'H',
        long,
        default_value = "0.0.0.0",
        env = "NOMS_SERVER_API_HOST"
    )]
    pub host: String,

    /// The JWT secret aka the secret key for generating JWT tokens.
    ///
    /// Should be a random string of decent length (30+ characters).
    ///
    /// If not specified, a random one will be generated.
    ///
    /// Every time it's changed, all existing sessions will be invalidated.
    ///
    /// If application is run on multiple machines,
    /// you should use the same value for all of them
    /// to ensure that sessions are valid across all servers.
    #[clap(short = 'S', long, default_value_t = random_jwt_secret(), env = "NOMS_SERVER_API_JWT_SECRET", hide_env_values = true, hide_default_value = true)]
    pub jwt_secret: String,
}

impl ApiServerConfig {
    pub fn host_address(&self) -> Result<SocketAddr, AddrParseError> {
        let host = self.host.parse::<std::net::IpAddr>()?;

        Ok(SocketAddr::new(host, self.port))
    }
}

#[derive(Debug, Clone, clap::Args, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AdminServerConfig {
    /// The port the admin server listens on.
    #[clap(
        name = "admin-port",
        long,
        default_value = "9099",
        env = "NOMS_SERVER_ADMIN_PORT"
    )]
    pub port: u16,

    /// The host to bind the admin server to.
    ///
    /// Should usually be either `0.0.0.0` if you want to bind to all interfaces aka the public,
    /// or `127.0.0.1` if you don't want to expose the server to the outside world.
    #[clap(
        name = "admin-host",
        long,
        default_value = "127.0.0.1",
        env = "NOMS_SERVER_ADMIN_HOST"
    )]
    pub host: String,

    /// If set to true, the admin server will not be started.
    #[clap(long, env = "NOMS_SERVER_ADMIN_DONT_RUN", default_value = "false")]
    pub no_run_admin: bool,
}

impl AdminServerConfig {
    pub fn address(&self) -> Result<SocketAddr, AddrParseError> {
        let host = self.host.parse::<std::net::IpAddr>()?;

        Ok(SocketAddr::new(host, self.port))
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum DatabaseUrl {
    Memory,
    Local(PathBuf),
    Remote {
        url: url::Url,
        token: Option<String>,
    },
}
impl DatabaseUrl {
    fn try_from_string(s: &str) -> Result<Self, String> {
        if s == ":memory:" {
            return Ok(Self::Memory);
        }

        match url::Url::parse(s) {
            Ok(mut url) => {
                match url.scheme() {
                    "sqlite" | "sqlite3" | "file" => {
                        return Ok(Self::Local(PathBuf::from(url.path())));
                    }
                    "http" | "https" | "libsql" | "ws" | "wss" => {}
                    _ => {
                        return Err("Invalid database URL scheme. Expected sqlite, sqlite3, file, http, https, libsql, ws or wss.".into());
                    }
                }

                let query_params = url.query_pairs().collect::<Vec<_>>();

                let token_pair = query_params
                    .iter()
                    .filter(|x| !x.1.is_empty())
                    .find(|x| {
                        matches!(
                            x.0.to_lowercase().as_str(),
                            "token" | "auth_token" | "authtoken"
                        )
                    })
                    .map(|x| (x.0.to_string(), x.1.to_string()));

                if let Some((token_name, _)) = token_pair.as_ref() {
                    let cleaned_pairs = query_params
                        .iter()
                        .filter(|x| &x.0 != token_name)
                        .map(|(x, y)| (x.to_string(), y.to_string()))
                        .collect::<Vec<_>>();

                    url.query_pairs_mut().clear().extend_pairs(cleaned_pairs);
                }

                let token = if let Some((_, token)) = token_pair {
                    Some(token)
                } else {
                    None
                };

                Ok(Self::Remote { url, token })
            }
            Err(_) => Ok(Self::Local(PathBuf::from(s))),
        }
    }
}
impl std::fmt::Display for DatabaseUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Memory => write!(f, ":memory:"),
            Self::Local(path) => write!(f, "file://{}", path.display()),
            Self::Remote { .. } => write!(f, "remote://[redacted]"),
        }
    }
}

fn random_jwt_secret() -> String {
    let mut s = String::with_capacity(160);

    let now = jiff::Timestamp::now().as_nanosecond();

    let _ = write!(s, "{}-", now);

    for byte in rand::random_iter::<u8>().take(64) {
        let _ = write!(s, "{:02x}", byte);
    }

    s
}

impl std::fmt::Debug for ApiServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiServerConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("jwt_secret", &"[redacted]")
            .finish()
    }
}
impl std::fmt::Debug for DatabaseUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[derive(Clone)]
struct DatabaseUrlParser;
impl clap::builder::TypedValueParser for DatabaseUrlParser {
    type Value = DatabaseUrl;
    fn parse_ref(
        &self,
        _command: &clap::Command,
        _argument: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        value
            .to_str()
            .and_then(|value| DatabaseUrl::try_from_string(value).ok())
            .ok_or_else(|| {
                clap::Error::raw(
                    clap::error::ErrorKind::ValueValidation,
                    "Invalid database URL (input redacted)",
                )
            })
    }
}
