#![allow(clippy::unwrap_used, clippy::expect_used)]

use app_services::{
    Client, Notification, Registry, ServiceUrl,
    service::{PreparedService, Service, ServiceParseError, ServiceSendError},
};
use std::{path::PathBuf, sync::Arc};

#[derive(Debug)]
struct FileProvider(PathBuf);
struct FileSender(PathBuf);
impl Service for FileProvider {
    fn name(&self) -> &'static str {
        "file"
    }
    fn description(&self) -> &'static str {
        "Write a notification"
    }
    fn prepare(&self, url: &ServiceUrl) -> Result<Box<dyn PreparedService>, ServiceParseError> {
        Ok(Box::new(FileSender(self.0.join(&url.configuration))))
    }
}
#[async_trait::async_trait]
impl PreparedService for FileSender {
    async fn send(&self, notification: &Notification) -> Result<(), ServiceSendError> {
        std::fs::write(&self.0, &notification.message).map_err(|e| ServiceSendError::Send {
            url: "file".into(),
            source: e.into(),
        })
    }
}

#[tokio::test]
async fn custom_providers_receive_only_destinations_matching_all_tags() {
    let dir = tempfile::tempdir().unwrap();
    let mut registry = Registry::new();
    registry.register(Arc::new(FileProvider(dir.path().to_path_buf())));
    let mut client = Client::new(registry);
    for url in [
        "file://selected#tag[]=public&tag[]=urgent",
        "file://private#tag[]=private",
        "file://ordinary#tag[]=public",
    ] {
        client.add_url(&url.parse().unwrap()).unwrap();
    }
    let notification = Notification {
        title: None,
        message: "delivered".into(),
        tags: vec!["public".into(), "urgent".into()],
    };
    let results = client.send(&notification).await;
    assert_eq!(results.len(), 1);
    assert!(results[0].result.is_ok());
    assert_eq!(
        std::fs::read_to_string(dir.path().join("selected")).unwrap(),
        "delivered"
    );
    assert!(!dir.path().join("private").exists());
    assert!(!dir.path().join("ordinary").exists());
}

#[tokio::test]
async fn dispatch_reports_partial_failures_and_continues_to_other_destinations() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("cannot-write-file")).unwrap();
    let mut registry = Registry::new();
    registry.register(Arc::new(FileProvider(dir.path().to_path_buf())));
    let mut client = Client::new(registry);
    for name in ["cannot-write-file", "delivered"] {
        client
            .add_url(&format!("file://{name}").parse().unwrap())
            .unwrap();
    }
    let reports = client
        .send(&Notification {
            title: None,
            message: "hello".into(),
            tags: vec![],
        })
        .await;
    assert_eq!(reports.len(), 2);
    assert_eq!(reports.iter().filter(|r| r.result.is_err()).count(), 1);
    assert_eq!(
        std::fs::read_to_string(dir.path().join("delivered")).unwrap(),
        "hello"
    );
}
