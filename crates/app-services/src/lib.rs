//! Reusable notification destinations and tag-aware dispatch.
//!
//! ```no_run
//! use app_services::{Client, Notification};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut client = Client::default();
//! client.add_url(&"tgram://123:token/-1001234567890#tag[]=alerts".parse()?)?;
//! let reports = client.send(&Notification {
//!     title: None,
//!     message: "Something happened.".into(),
//!     tags: vec!["alerts".into()],
//! }).await;
//! for report in reports { report.result?; }
//! # Ok(())
//! # }
//! ```
//!
//! Built-in providers are controlled by the `email` and `telegram` features,
//! both enabled by default. See [`Registry`] for custom provider registration.

pub mod notification;
pub mod service;
pub mod tag;

mod client;

pub use client::{Client, DeliveryReport, Registry};

pub use notification::Notification;
pub use service::ServiceUrl;
pub use tag::Tag;
