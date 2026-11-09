pub mod send;

#[derive(Debug, clap::Subcommand, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunCommand {
    /// Send a notification to the specified service(s)
    ///
    /// eg. `noms send -u mailto://user:pass@example.com -m "Hello, world!"`
    Send(send::SendConfig),

    /// Start the noms server
    ///
    /// The server acts as a stateful sender, which can be used to send notifications
    /// from network-connected services.
    ///
    /// Can be thought of as somewhat like a stateful CLI, but with some authentication built in.
    #[cfg(feature = "server")]
    Server(app_server::ServerConfig),
}
