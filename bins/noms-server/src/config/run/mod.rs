#[derive(Debug, clap::Subcommand, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunCommand {
    /// Start the noms server
    ///
    /// The server acts as a stateful sender, which can be used to send notifications
    /// from network-connected services.
    ///
    /// Can be thought of as somewhat like a stateful CLI, but with some authentication built in.
    Server(app_server::ServerConfig),
}
