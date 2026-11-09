use lettre::{
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
    message::{Mailbox, Mailboxes, header::ContentType},
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};

use crate::{
    Notification, ServiceUrl,
    service::{
        PreparedService, Service, ServiceParseError, ServiceSendError, config_error,
        service_url::DownstreamFormat,
    },
};

#[derive(Debug, Default)]
pub struct EmailService;

impl Service for EmailService {
    fn name(&self) -> &'static str {
        "email"
    }
    fn description(&self) -> &'static str {
        "Send an email"
    }
    fn can_handle(&self, name: &str) -> bool {
        matches!(name, "mailto" | "email" | "smtp" | "smtps")
    }

    fn prepare(&self, url: &ServiceUrl) -> Result<Box<dyn PreparedService>, ServiceParseError> {
        let parsed = url::Url::parse(&url.original_url)
            .map_err(|_| config_error(url, "Invalid SMTP URL"))?;
        let host = parsed
            .host_str()
            .ok_or_else(|| config_error(url, "SMTP URL requires a host"))?;
        let tls_mode = match url.parameters.get("tls") {
            None => {
                if parsed.scheme() == "smtps" {
                    "wrapper"
                } else {
                    "required"
                }
            }
            Some(either::Left(s)) => s.as_str(),
            Some(either::Right(_)) => {
                return Err(config_error(url, "tls must have a single value"));
            }
        };
        if parsed.scheme() == "smtps" && tls_mode != "wrapper" {
            return Err(config_error(url, "smtps requires implicit TLS"));
        }
        let tls = match tls_mode {
            "none" => Tls::None,
            "required" | "wrapper" => {
                let parameters = TlsParameters::builder(host.to_string())
                    .dangerous_accept_invalid_certs(!url.settings.verify_certificates)
                    .dangerous_accept_invalid_hostnames(!url.settings.verify_certificates)
                    .build()
                    .map_err(|_| config_error(url, "Cannot configure SMTP TLS"))?;
                if tls_mode == "wrapper" {
                    Tls::Wrapper(parameters)
                } else {
                    Tls::Required(parameters)
                }
            }
            _ => {
                return Err(config_error(
                    url,
                    "tls must be required, wrapper, or none (explicit insecure opt-in)",
                ));
            }
        };
        let mut builder = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(host)
            .port(
                parsed
                    .port()
                    .unwrap_or(if tls_mode == "wrapper" { 465 } else { 587 }),
            )
            .tls(tls)
            .timeout(Some(url.settings.connection_timeout));
        if !parsed.username().is_empty() || parsed.password().is_some() {
            let username = urlencoding::decode(parsed.username())
                .map_err(|_| config_error(url, "Invalid SMTP username encoding"))?
                .into_owned();
            let password = urlencoding::decode(parsed.password().unwrap_or_default())
                .map_err(|_| config_error(url, "Invalid SMTP password encoding"))?
                .into_owned();
            builder = builder.credentials(Credentials::new(username, password));
        }
        let from = mailboxes(url, "from", true)?;
        if from.len() != 1 {
            return Err(config_error(url, "from must contain exactly one address"));
        }
        let reply_to = mailboxes(url, "reply-to", false)?;
        let to = mailboxes(url, "to", true)?;
        let cc = mailboxes(url, "cc", false)?;
        let bcc = mailboxes(url, "bcc", false)?;
        Ok(Box::new(EmailSender {
            transport: builder.build(),
            from: from[0].clone(),
            to,
            cc,
            bcc,
            reply_to,
            format: url.settings.format.clone(),
            destination: url.to_string(),
        }))
    }
}

struct EmailSender {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
    to: Vec<Mailbox>,
    cc: Vec<Mailbox>,
    bcc: Vec<Mailbox>,
    reply_to: Vec<Mailbox>,
    format: Option<DownstreamFormat>,
    destination: String,
}

#[async_trait::async_trait]
impl PreparedService for EmailSender {
    async fn send(&self, notification: &Notification) -> Result<(), ServiceSendError> {
        let mut builder = lettre::Message::builder().from(self.from.clone());
        for mailbox in &self.to {
            builder = builder.to(mailbox.clone());
        }
        for mailbox in &self.cc {
            builder = builder.cc(mailbox.clone());
        }
        for mailbox in &self.bcc {
            builder = builder.bcc(mailbox.clone());
        }
        for mailbox in &self.reply_to {
            builder = builder.reply_to(mailbox.clone());
        }
        if let Some(title) = &notification.title {
            builder = builder.subject(title);
        }
        let content_type = if self.format == Some(DownstreamFormat::Html) {
            ContentType::TEXT_HTML
        } else {
            ContentType::TEXT_PLAIN
        };
        let message = builder
            .header(content_type)
            .body(notification.message.clone())
            .map_err(|e| ServiceSendError::FailedToSerializeBody(e.into()))?;
        self.transport
            .send(message)
            .await
            .map_err(|_| ServiceSendError::Send {
                url: self.destination.clone(),
                source: anyhow::anyhow!("SMTP delivery failed"),
            })?;
        Ok(())
    }
}

fn mailboxes(
    url: &ServiceUrl,
    key: &str,
    required: bool,
) -> Result<Vec<Mailbox>, ServiceParseError> {
    let Some(value) = url.parameters.get(key) else {
        return if required {
            Err(config_error(url, format!("Missing {key} query parameter")))
        } else {
            Ok(vec![])
        };
    };
    let values: Vec<&str> = match value {
        either::Left(s) => vec![s],
        either::Right(values) => values.iter().map(String::as_str).collect(),
    };
    let mut mailboxes = vec![];
    for value in values {
        let parsed: Mailboxes = value
            .parse()
            .map_err(|_| config_error(url, format!("Invalid address in {key}")))?;
        mailboxes.extend(parsed);
    }
    if mailboxes.is_empty() {
        return Err(config_error(url, format!("{key} must contain an address")));
    }
    Ok(mailboxes)
}
