use std::{collections::HashMap, str::FromStr, sync::LazyLock};

use either::Either;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::Tag;

use std::time::Duration;

/// Service URL that contains all the required information to send a notification.
///
/// Format: ${SERVICE}://${CONFIGURATION}/${ARGUMENTS}?${PARAMETERS}#${FRAGMENT}
#[derive(Clone, PartialEq, Eq)]
pub struct ServiceUrl {
    pub name: String,
    pub original_url: String,
    pub configuration: String,
    pub parameters: HashMap<String, Either<String, Vec<String>>>,
    pub arguments: Vec<String>,
    pub settings: ServiceSettings,
}

impl ServiceUrl {
    #[cfg(feature = "telegram")]
    pub fn as_reqwest_client(&self) -> Result<reqwest::Client, reqwest::Error> {
        reqwest::Client::builder()
            .connect_timeout(self.settings.connection_timeout)
            .timeout(self.settings.connection_timeout)
            .tls_danger_accept_invalid_certs(!self.settings.verify_certificates)
            .redirect(reqwest::redirect::Policy::limited(
                self.settings.redirect_limit,
            ))
            .user_agent(self.settings.user_agent.clone())
            .build()
    }

    #[must_use]
    pub fn get_parameter<'a, 'b: 'a>(
        &'a self,
        key: &'b str,
    ) -> Option<&'a Either<String, Vec<String>>> {
        self.parameters.get(key)
    }
}

impl Serialize for ServiceUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.original_url)
    }
}

impl<'de> Deserialize<'de> for ServiceUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let url = String::deserialize(deserializer)?;

        Self::from_str(&url).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl FromStr for ServiceUrl {
    type Err = ServiceUrlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static SERVICE_URL_RE: LazyLock<Regex> = LazyLock::new(|| {
            #[allow(clippy::expect_used)]
            Regex::new(
                r"^(?P<service>[^:\s]+)://(?P<configuration>[^/?#]+)?(/(?P<arguments>[^?#]+)?)?(\?(?P<parameters>[^#]+)?)?(#(?P<fragment>.+)?)?$",
            )
            .expect(
    "Failed to compile regex")
        });

        let Some(caps) = SERVICE_URL_RE.captures(s) else {
            return Err(ServiceUrlError::UrlFormat);
        };

        let service = caps.name("service").map(|x| x.as_str()).unwrap_or_default();
        let configuration = caps
            .name("configuration")
            .map(|x| x.as_str())
            .unwrap_or_default();

        let parameters = {
            let s = caps
                .name("parameters")
                .map(|x| x.as_str())
                .unwrap_or_default();

            if s.is_empty() {
                #[allow(clippy::default_trait_access)]
                Default::default()
            } else {
                let qs_items: HashMap<String, serde_json::Value> =
                    serde_qs::from_str(s).map_err(|_| ServiceUrlError::ParseParameters)?;

                qs_items
                    .into_iter()
                    .filter_map(|(k, v)| {
                        if let serde_json::Value::String(s) = v {
                            return Some((k, Either::Left(s)));
                        }

                        if let serde_json::Value::Array(a) = v {
                            return Some((
                                k,
                                Either::Right(
                                    a.into_iter()
                                        .filter_map(|x| {
                                            x.as_str().map(std::string::ToString::to_string)
                                        })
                                        .collect(),
                                ),
                            ));
                        }

                        None
                    })
                    .collect()
            }
        };
        let arguments = caps
            .name("arguments")
            .map(|x| x.as_str())
            .map(|x| {
                x.split('/')
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let fragment = caps
            .name("fragment")
            .map(|x| x.as_str())
            .unwrap_or_default();

        let settings = serde_qs::from_str(fragment).map_err(|_| ServiceUrlError::ParseSettings)?;

        let settings: ServiceSettings = settings;
        if settings.connection_timeout.is_zero() {
            return Err(ServiceUrlError::ParseSettings);
        }

        Ok(Self {
            name: service.to_string(),
            original_url: s.to_string(),
            configuration: configuration.to_string(),
            parameters,
            arguments,
            settings,
        })
    }
}

impl std::fmt::Display for ServiceUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://[redacted]", self.name)
    }
}

impl std::fmt::Debug for ServiceUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceUrlError {
    #[error("Invalid service URL; expected <scheme>://<configuration>/<arguments>")]
    UrlFormat,
    #[error("Invalid service URL query parameters")]
    ParseParameters,
    #[error("Invalid service URL settings")]
    ParseSettings,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ServiceSettings {
    #[serde(alias = "tag")]
    #[serde(alias = "t", default = "default_tags")]
    pub tags: Vec<Tag>,

    /// Some services support different formats of messages.
    /// This field specifies the format of the message that will be sent to the service.
    ///
    /// If not specified, the default format is used.
    /// If the downstream service doesn't support the specified format, the message will be sent in the default format.
    /// If the service doesn't support formats at all, this field is ignored.
    #[serde(alias = "f")]
    pub format: Option<DownstreamFormat>,

    /// Whether to verify the server's certificate.
    ///
    /// If not specified, the default value is `true`.
    #[serde(alias = "verify", default = "default_verify_certificates")]
    pub verify_certificates: bool,

    /// The timeout for the connection to the service.
    ///
    /// Bounds both connection establishment and the complete send request.
    /// Must be greater than zero.
    /// The default value is 4 seconds.
    #[serde(alias = "timeout", default = "default_connection_timeout")]
    #[serde(with = "jiff::fmt::serde::unsigned_duration::required")]
    pub connection_timeout: Duration,

    /// How many redirects to follow.
    ///
    /// If not specified, the default value is `5`.
    /// Set to `0` to disable redirects.
    #[serde(alias = "redirects", default = "default_redirect_limit")]
    pub redirect_limit: usize,

    /// The user agent to use when sending requests.
    ///
    /// Defaults to `NomNomNom (noms)`.
    #[serde(alias = "ua", default = "default_user_agent")]
    pub user_agent: String,
}

#[must_use]
pub fn default_user_agent() -> String {
    "NomNomNom (noms)".to_string()
}

#[must_use]
pub fn default_redirect_limit() -> usize {
    5
}

#[must_use]
pub fn default_tags() -> Vec<Tag> {
    Vec::new()
}

#[must_use]
pub fn default_verify_certificates() -> bool {
    true
}

#[must_use]
pub fn default_connection_timeout() -> Duration {
    Duration::from_secs(4)
}

/// Some services support different formats of messages.
/// This enum specifies the format of the message that will be sent to the service.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DownstreamFormat {
    Text,
    Html,
    Markdown,
}

impl Default for ServiceSettings {
    fn default() -> Self {
        Self {
            tags: default_tags(),
            format: None,
            verify_certificates: default_verify_certificates(),
            connection_timeout: default_connection_timeout(),
            redirect_limit: default_redirect_limit(),
            user_agent: default_user_agent(),
        }
    }
}
