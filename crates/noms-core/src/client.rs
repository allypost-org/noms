use std::sync::Arc;

use futures_util::{StreamExt, stream};

use crate::{
    Notification, ServiceUrl,
    service::{
        AVAILABLE_SERVICES, AssociatedService, Service, ServiceParseError, ServiceSendError,
    },
};

/// Selects a provider for each URL. Register custom providers before preparing destinations.
#[derive(Debug)]
pub struct Registry {
    services: Vec<Arc<dyn Service>>,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            services: AVAILABLE_SERVICES.clone(),
        }
    }
}

impl Registry {
    /// Create an empty registry, without built-in providers.
    #[must_use]
    pub fn new() -> Self {
        Self { services: vec![] }
    }

    /// Register a provider, replacing any existing provider with the same name.
    pub fn register(&mut self, service: Arc<dyn Service>) {
        self.services
            .retain(|existing| existing.name() != service.name());
        self.services.push(service);
    }

    pub fn prepare(&self, url: &ServiceUrl) -> Result<AssociatedService, ServiceParseError> {
        self.services
            .iter()
            .find_map(|service| url.associate_if_valid(service.as_ref()))
            .unwrap_or(Err(ServiceParseError::Unsupported))
    }
}

/// Reusable destinations with bounded concurrent dispatch and AND matching of notification tags.
#[derive(Debug, Default)]
pub struct Client {
    registry: Registry,
    destinations: Vec<AssociatedService>,
}

#[derive(Debug)]
pub struct DeliveryReport {
    pub destination: String,
    pub result: Result<(), ServiceSendError>,
}

impl Client {
    #[must_use]
    pub fn new(registry: Registry) -> Self {
        Self {
            registry,
            destinations: vec![],
        }
    }

    /// Validate and prepare a destination without connecting to it.
    /// Keep this client for later sends to reuse its transports.
    pub fn add_url(&mut self, url: &ServiceUrl) -> Result<(), ServiceParseError> {
        self.destinations.push(self.registry.prepare(url)?);
        Ok(())
    }

    /// Deliver to all selected destinations. Reports may arrive in completion order.
    /// A failed destination does not cancel other deliveries. No retries are performed.
    /// Inspect each report: completion does not imply success. No matching tags
    /// produces an empty vector. Provider futures require a Tokio runtime.
    pub async fn send(&self, notification: &Notification) -> Vec<DeliveryReport> {
        let deliveries = self
            .destinations
            .iter()
            .filter(|destination| destination.matches_tags(&notification.tags))
            .map(|destination| async move {
                DeliveryReport {
                    destination: destination.name_with_tags(),
                    result: destination.send(notification).await,
                }
            })
            .collect::<Vec<_>>();

        stream::iter(deliveries)
            .buffer_unordered(16)
            .collect()
            .await
    }
}
