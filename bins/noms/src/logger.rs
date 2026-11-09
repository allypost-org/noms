#![allow(dead_code)]

use std::{env, sync::OnceLock};

use owo_colors::OwoColorize;
use tracing::{Level, warn};
use tracing_subscriber::{EnvFilter, filter::Directive, fmt, prelude::*};

static RELOAD_HANDLE: OnceLock<
    tracing_subscriber::reload::Handle<
        EnvFilter,
        tracing_subscriber::layer::Layered<
            fmt::Layer<tracing_subscriber::Registry>,
            tracing_subscriber::Registry,
        >,
    >,
> = OnceLock::new();

pub const COMPONENT_LEVELS: &[(&str, Level)] = &[
    // Binaries
    ("noms", Level::WARN),
    // Libraries
    ("app_services", Level::INFO),
    ("app_server", Level::INFO),
    // Other
    ("app", Level::INFO),
    ("request", Level::INFO),
    // External
];

/// Initialize the logger
///
/// # Panics
/// Panics if the logger fails to initialize
pub fn init() {
    init_with(COMPONENT_LEVELS.to_vec());
}

pub fn init_with<T>(levels: T)
where
    T: IntoIterator<Item = (&'static str, Level)>,
{
    let default_levels = levels
        .into_iter()
        .map(|(k, v)| {
            if k.is_empty() {
                v.to_string()
            } else {
                format!("{}={}", k, v)
            }
        })
        .fold(String::new(), |acc, a| format!("{},{}", acc, a));

    let mut base_level = EnvFilter::builder()
        .with_default_directive(Level::WARN.into())
        .parse_lossy(default_levels);

    let env_directives = env::var("LOG_LEVEL")
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .filter_map(|s| match s.parse() {
            Ok(d) => Some(d),
            Err(e) => {
                eprintln!("Failed to parse log level directive {s:?}: {e:?}");
                None
            }
        })
        .collect::<Vec<Directive>>();

    for d in env_directives {
        base_level = base_level.add_directive(d);
    }

    let (base_level, reload_handle) = tracing_subscriber::reload::Layer::new(base_level);
    #[allow(clippy::expect_used)]
    RELOAD_HANDLE
        .set(reload_handle)
        .expect("Logger was already initialized");

    #[allow(clippy::expect_used)]
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(base_level)
        .try_init()
        .expect("setting default subscriber failed");
}

pub fn update_log_level(log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
    let default_levels = COMPONENT_LEVELS
        .iter()
        .map(|(k, v)| {
            if k.is_empty() {
                v.to_string()
            } else {
                format!("{}={}", k, v)
            }
        })
        .fold(String::new(), |acc, a| format!("{},{}", acc, a));

    let mut base_level = EnvFilter::builder()
        .with_default_directive(Level::WARN.into())
        .parse_lossy(default_levels);

    let env_directives = log_level
        .split(',')
        .filter(|s| !s.is_empty())
        .filter_map(|s| match s.parse() {
            Ok(d) => Some(d),
            Err(err) => {
                warn!(directive = ?s, ?err, "Failed to parse log level directive");
                None
            }
        })
        .collect::<Vec<Directive>>();

    for d in env_directives {
        base_level = base_level.add_directive(d);
    }

    #[allow(clippy::expect_used)]
    let reload_handle = RELOAD_HANDLE.get().expect("Logger was not initialized");

    reload_handle
        .modify(|filter| *filter = base_level)
        .map_err(|e| format!("Failed to set log level: {:?}", e).into())
}

pub enum PrettyLog {
    Debug(String),
    Notice(String),
    Info(String),
    Warn(String),
    Error(String),
}

impl PrettyLog {
    pub fn log(&self) {
        println!("{}", self);
    }

    pub fn debug(msg: &str) {
        Self::Debug(msg.to_string()).log();
    }

    pub fn notice(msg: &str) {
        Self::Notice(msg.to_string()).log();
    }

    pub fn info(msg: &str) {
        Self::Info(msg.to_string()).log();
    }

    pub fn warn(msg: &str) {
        Self::Warn(msg.to_string()).log();
    }

    pub fn error(msg: &str) {
        Self::Error(msg.to_string()).log();
    }
}

impl std::fmt::Display for PrettyLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debug(msg) => write!(
                f,
                "{} {}{}",
                " DEBUG".if_supports_color(owo_colors::Stream::Stdout, |x| x
                    .white()
                    .on_black()
                    .to_string()),
                ":".if_supports_color(owo_colors::Stream::Stdout, |x| x
                    .black()
                    .on_black()
                    .to_string()),
                msg,
            ),
            Self::Notice(msg) => write!(
                f,
                "{}{} {}",
                " NOTE".if_supports_color(owo_colors::Stream::Stdout, |x| x
                    .bold()
                    .bright_white()
                    .on_bright_black()
                    .to_string()),
                ":".if_supports_color(owo_colors::Stream::Stdout, |x| x
                    .bold()
                    .bright_black()
                    .on_bright_black()
                    .to_string()),
                msg,
            ),
            Self::Info(msg) => write!(
                f,
                "{}{} {}",
                " INFO".if_supports_color(owo_colors::Stream::Stdout, |x| x
                    .black()
                    .on_green()
                    .to_string()),
                ":".if_supports_color(owo_colors::Stream::Stdout, |x| x
                    .green()
                    .on_green()
                    .to_string()),
                msg,
            ),
            Self::Warn(msg) => write!(
                f,
                "{}{} {}",
                " WARN".if_supports_color(owo_colors::Stream::Stdout, |x| x
                    .bold()
                    .black()
                    .on_yellow()
                    .to_string()),
                ":".if_supports_color(owo_colors::Stream::Stdout, |x| x
                    .bold()
                    .yellow()
                    .on_yellow()
                    .to_string()),
                msg,
            ),
            Self::Error(msg) => write!(
                f,
                "{}{} {}",
                " ERROR".if_supports_color(owo_colors::Stream::Stdout, |x| x
                    .bold()
                    .bright_white()
                    .on_red()
                    .to_string()),
                ":".if_supports_color(owo_colors::Stream::Stdout, |x| x
                    .bold()
                    .red()
                    .on_red()
                    .to_string()),
                msg,
            ),
        }
    }
}

pub fn p_debug(msg: &str) {
    PrettyLog::debug(msg);
}

pub fn p_notice(msg: &str) {
    PrettyLog::notice(msg);
}

pub fn p_info(msg: &str) {
    PrettyLog::info(msg);
}

pub fn p_warn(msg: &str) {
    PrettyLog::warn(msg);
}

pub fn p_err(msg: &str) {
    PrettyLog::error(msg);
}
