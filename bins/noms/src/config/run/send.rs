use app_services::{Notification, ServiceUrl, Tag};

#[derive(Debug, clap::Args, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SendConfig {
    /// The service URL to use. Can be specified multiple times.
    ///
    /// eg. `mailto://user:pass@example.com?to=user@example.net`
    #[arg(
        short = 'u',
        long = "service-url",
        required = true,
        value_parser = ServiceUrlParser,
        hide_env_values = true,
        env = "NOMS_SERVICE_URL"
    )]
    pub service_urls: Vec<ServiceUrl>,

    /// The title of the notification message
    #[arg(short, long, env = "NOMS_SEND_TITLE")]
    pub title: Option<String>,

    /// The message to show in the notification
    #[arg(short, long, env = "NOMS_SEND_MESSAGE")]
    pub message: String,

    /// Exit with an error if any destination fails.
    ///
    /// Validation errors prevent sending. Once sending starts, all selected
    /// destinations are attempted; partial failures otherwise allow success.
    #[arg(short, long, env = "NOMS_SEND_ABORT_ON_ERROR")]
    pub abort_on_error: bool,

    /// Validate and list selected destinations without connecting or sending.
    #[arg(long, env = "NOMS_SEND_DRY_RUN")]
    #[serde(default)]
    pub dry_run: bool,

    /// If set, only service URLs tagged with these tags will be used.
    /// Can be specified multiple times for AND-ing tags.
    ///
    /// eg. `--tag foo --tag bar` means "only URLs tagged with both `foo` and `bar` will be used".
    #[arg(
        short = 'T',
        long = "tag",
        value_delimiter = ',',
        env = "NOMS_SEND_TAGS"
    )]
    pub tags: Vec<Tag>,
}

impl SendConfig {
    pub fn into_notification(self) -> Notification {
        Notification {
            title: self.title,
            message: self.message,
            tags: self.tags,
        }
    }
}

#[derive(Clone)]
struct ServiceUrlParser;
impl clap::builder::TypedValueParser for ServiceUrlParser {
    type Value = ServiceUrl;
    fn parse_ref(
        &self,
        _command: &clap::Command,
        _argument: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        value
            .to_str()
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| {
                clap::Error::raw(
                    clap::error::ErrorKind::ValueValidation,
                    "Invalid service URL (input redacted)",
                )
            })
    }
}
