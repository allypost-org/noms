use std::{fmt, sync::Arc};

use thiserror::Error;
use tracing::Instrument;

use crate::Notification;

pub use impls::AVAILABLE_SERVICES;
#[cfg(feature = "email")]
pub use impls::email::EmailService;
#[cfg(feature = "telegram")]
pub use impls::telegram::TelegramService;
pub use service_url::ServiceUrl;

mod impls;
pub mod service_url;

pub type ServiceEntry = Arc<dyn Service>;

/// A validated destination with its reusable transport and typed configuration.
pub struct AssociatedService {
    name: &'static str,
    description: &'static str,
    url: ServiceUrl,
    sender: Box<dyn PreparedService>,
}

impl fmt::Debug for AssociatedService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AssociatedService")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl AssociatedService {
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub fn description(&self) -> &'static str {
        self.description
    }

    #[must_use]
    pub fn name_with_tags(&self) -> String {
        let tags = &self.url.settings.tags;
        if tags.is_empty() {
            return self.name().to_string();
        }
        format!("{} ({})", self.name(), tags.join(", "))
    }

    #[must_use]
    pub fn matches_tags(&self, tags: &[crate::Tag]) -> bool {
        tags.iter().all(|tag| self.url.settings.tags.contains(tag))
    }

    pub async fn send(&self, notification: &Notification) -> Result<(), ServiceSendError> {
        self.sender
            .send(notification)
            .instrument(tracing::debug_span!("send", service = self.name))
            .await
    }
}

impl ServiceUrl {
    pub fn associate_if_valid(
        &self,
        service: &dyn Service,
    ) -> Option<Result<AssociatedService, ServiceParseError>> {
        if !service.can_handle(&self.name) {
            return None;
        }
        Some(service.prepare(self).map(|sender| AssociatedService {
            name: service.name(),
            description: service.description(),
            url: self.clone(),
            sender,
        }))
    }
}

/// Extension point for notification providers. Serialization is kept out of the trait.
pub trait Service: fmt::Debug + Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn can_handle(&self, scheme: &str) -> bool {
        self.name() == scheme
    }
    fn prepare(&self, url: &ServiceUrl) -> Result<Box<dyn PreparedService>, ServiceParseError>;
}

/// A provider's validated, typed send operation.
#[async_trait::async_trait]
pub trait PreparedService: Send + Sync {
    async fn send(&self, notification: &Notification) -> Result<(), ServiceSendError>;
}

#[derive(Debug, Error)]
pub enum ServiceSendError {
    #[error("Failed to build transport: {0}")]
    FailedToBuildTransport(anyhow::Error),
    #[error("Failed to serialize body: {0}")]
    FailedToSerializeBody(anyhow::Error),
    #[error("Failed sending to {url}: {source}")]
    Send { url: String, source: anyhow::Error },
    #[error("Error response from {url}: {source}")]
    Response { url: String, source: anyhow::Error },
    #[error("Invalid response from {url}: {source}")]
    ParsingResponse { url: String, source: anyhow::Error },
    #[error("Rejected by {url}: {message}")]
    ResponseRejected { url: String, message: String },
}

#[derive(Debug, Error)]
pub enum ServiceParseError {
    #[error("Unsupported notification service")]
    Unsupported,
    #[error("Invalid configuration: {0}")]
    Configuration(ConfigurationError),
    #[error("Invalid parameters: {0}")]
    Parameters(ParametersError),
    #[error("Invalid arguments: {0}")]
    Arguments(#[from] ArgumentsError),
}

#[derive(Debug)]
pub struct ConfigurationError {
    /// A redacted destination identifier, never credentials.
    pub configuration: String,
    pub message: String,
}
impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

#[derive(Debug, Error)]
pub enum ParametersError {
    #[error("Problem with parameter {parameter}: {message}")]
    Parameter { parameter: String, message: String },
    #[error("Problem with parameters {parameters:?}: {message}")]
    Parameters {
        parameters: Vec<String>,
        message: String,
    },
}

#[derive(Debug, Error)]
pub enum ArgumentsError {
    #[error("Problem with argument {argument}: {message}")]
    Argument { argument: String, message: String },
    #[error("Problem with arguments {arguments:?}: {message}")]
    Arguments {
        arguments: Vec<String>,
        message: String,
    },
    #[error("Not enough arguments. {0}")]
    NotEnoughArguments(String),
}

#[cfg(any(feature = "email", feature = "telegram"))]
pub(crate) fn config_error(url: &ServiceUrl, message: impl Into<String>) -> ServiceParseError {
    ServiceParseError::Configuration(ConfigurationError {
        configuration: url.to_string(),
        message: message.into(),
    })
}
