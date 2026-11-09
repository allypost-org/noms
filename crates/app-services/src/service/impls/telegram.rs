use std::{sync::LazyLock, time::Duration};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    Notification, ServiceUrl,
    service::{
        PreparedService, Service, ServiceParseError, ServiceSendError, config_error,
        service_url::DownstreamFormat,
    },
};

/// Telegram Bot API provider. Supports a custom API base for self-hosted Bot API servers.
#[derive(Clone)]
pub struct TelegramService {
    api_base: url::Url,
    client: Option<reqwest::Client>,
}

impl std::fmt::Debug for TelegramService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TelegramService")
    }
}

impl Default for TelegramService {
    fn default() -> Self {
        #[allow(clippy::expect_used)]
        Self {
            api_base: url::Url::parse("https://api.telegram.org/").expect("static URL"),
            client: None,
        }
    }
}

impl TelegramService {
    /// Configure a self-hosted Telegram Bot API server. Plain HTTP is permitted only when explicitly supplied here.
    #[must_use]
    pub fn with_api_base(mut self, api_base: url::Url) -> Self {
        self.api_base = api_base;
        self
    }

    /// Reuse a caller-owned client, including its TLS and proxy policy. Request deadlines still apply.
    #[must_use]
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }
}

impl Service for TelegramService {
    fn name(&self) -> &'static str {
        "telegram"
    }
    fn description(&self) -> &'static str {
        "Send a message to a Telegram chat"
    }
    fn can_handle(&self, name: &str) -> bool {
        matches!(name, "telegram" | "tgram")
    }

    fn prepare(&self, url: &ServiceUrl) -> Result<Box<dyn PreparedService>, ServiceParseError> {
        static TOKEN: LazyLock<Regex> = LazyLock::new(|| {
            #[allow(clippy::expect_used)]
            Regex::new(r"^[0-9]+:[A-Za-z0-9_-]+$").expect("static regex")
        });
        if !TOKEN.is_match(&url.configuration) {
            return Err(config_error(url, "Invalid Telegram bot token"));
        }
        if url.arguments.is_empty() {
            return Err(config_error(
                url,
                "At least one Telegram chat ID is required",
            ));
        }
        let chat_ids = url
            .arguments
            .iter()
            .map(|id| {
                if let Ok(value) = id.parse::<i64>() {
                    if value != 0 {
                        return Ok(ChatId::Number(value));
                    }
                } else if let Some(username) = id.strip_prefix('@')
                    && !username.is_empty()
                    && username
                        .bytes()
                        .all(|c| c.is_ascii_alphanumeric() || c == b'_')
                {
                    return Ok(ChatId::Username(id.clone()));
                }
                Err(config_error(
                    url,
                    "Chat ID must be a nonzero signed integer or @channel_username",
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let parse_mode = match url.parameters.get("parse_mode") {
            Some(either::Left(mode)) => match mode.as_str() {
                "" | "text" => None,
                "HTML" | "Markdown" | "MarkdownV2" => Some(mode.clone()),
                _ => {
                    return Err(config_error(
                        url,
                        "parse_mode must be text, HTML, Markdown, or MarkdownV2",
                    ));
                }
            },
            Some(either::Right(_)) => {
                return Err(config_error(url, "parse_mode must have a single value"));
            }
            None => match url.settings.format {
                Some(DownstreamFormat::Html) => Some("HTML".into()),
                Some(DownstreamFormat::Markdown) => Some("MarkdownV2".into()),
                _ => None,
            },
        };
        if !matches!(self.api_base.scheme(), "http" | "https") || self.api_base.host_str().is_none()
        {
            return Err(config_error(url, "Bot API base must be an HTTP(S) URL"));
        }
        let endpoint = self
            .api_base
            .join(&format!("./bot{}/sendMessage", url.configuration))
            .map_err(|_| config_error(url, "Invalid Bot API endpoint"))?;
        let client = match &self.client {
            Some(client) => client.clone(),
            None => url
                .as_reqwest_client()
                .map_err(|_| config_error(url, "Cannot configure Telegram HTTP client"))?,
        };
        Ok(Box::new(TelegramSender {
            client,
            endpoint,
            chat_ids,
            parse_mode,
            disable_preview: boolean(url, "disable_preview")?,
            disable_notification: boolean(url, "disable_notification")?,
            protect_content: boolean(url, "protect_content")?,
            timeout: url.settings.connection_timeout,
            destination: url.to_string(),
        }))
    }
}

struct TelegramSender {
    client: reqwest::Client,
    endpoint: url::Url,
    chat_ids: Vec<ChatId>,
    parse_mode: Option<String>,
    disable_preview: bool,
    disable_notification: bool,
    protect_content: bool,
    timeout: Duration,
    destination: String,
}

#[derive(Serialize)]
#[serde(untagged)]
enum ChatId {
    Number(i64),
    Username(String),
}

#[derive(Serialize)]
struct Message<'a> {
    chat_id: &'a ChatId,
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<&'a str>,
    link_preview_options: PreviewOptions,
    disable_notification: bool,
    protect_content: bool,
}
#[derive(Serialize)]
struct PreviewOptions {
    is_disabled: bool,
}

#[async_trait::async_trait]
impl PreparedService for TelegramSender {
    async fn send(&self, notification: &Notification) -> Result<(), ServiceSendError> {
        for chat_id in &self.chat_ids {
            let body = Message {
                chat_id,
                text: &notification.message,
                parse_mode: self.parse_mode.as_deref(),
                link_preview_options: PreviewOptions {
                    is_disabled: self.disable_preview,
                },
                disable_notification: self.disable_notification,
                protect_content: self.protect_content,
            };
            let response = self
                .client
                .post(self.endpoint.clone())
                .timeout(self.timeout)
                .json(&body)
                .send()
                .await
                .map_err(|e| ServiceSendError::Send {
                    url: self.destination.clone(),
                    source: e.without_url().into(),
                })?;
            let response = response
                .error_for_status()
                .map_err(|e| ServiceSendError::Response {
                    url: self.destination.clone(),
                    source: e.without_url().into(),
                })?;
            let response = response.json::<TelegramResponse>().await.map_err(|error| {
                // Decoder errors can contain upstream values, including reflected credentials.
                ServiceSendError::ParsingResponse {
                    url: self.destination.clone(),
                    source: anyhow::anyhow!(if error.is_timeout() {
                        "Timed out reading Telegram response"
                    } else {
                        "Invalid Telegram response"
                    }),
                }
            })?;
            if !response.ok {
                return Err(ServiceSendError::ResponseRejected {
                    url: self.destination.clone(),
                    message: "Telegram did not accept the message".into(),
                });
            }
        }
        Ok(())
    }
}

#[derive(Deserialize)]
struct TelegramResponse {
    ok: bool,
}

fn boolean(url: &ServiceUrl, key: &str) -> Result<bool, ServiceParseError> {
    match url.parameters.get(key) {
        None => Ok(false),
        Some(either::Left(value)) => match value.as_str() {
            "" | "true" | "1" => Ok(true),
            "false" | "0" => Ok(false),
            _ => Err(config_error(url, format!("{key} must be true or false"))),
        },
        Some(either::Right(_)) => Err(config_error(url, format!("{key} must have a single value"))),
    }
}
