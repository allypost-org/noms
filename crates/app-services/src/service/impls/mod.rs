#[cfg(any(feature = "email", feature = "telegram"))]
use std::sync::Arc;
use std::sync::LazyLock;

use super::ServiceEntry;

#[cfg(feature = "email")]
pub mod email;
#[cfg(feature = "telegram")]
pub mod telegram;

pub static AVAILABLE_SERVICES: LazyLock<Vec<ServiceEntry>> = LazyLock::new(|| {
    vec![
        #[cfg(feature = "email")]
        Arc::new(email::EmailService),
        #[cfg(feature = "telegram")]
        Arc::new(telegram::TelegramService::default()),
    ]
});
