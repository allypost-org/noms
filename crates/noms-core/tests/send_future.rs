//! Regression test: `Client::send` must return a `Send` future so it can be
//! spawned onto multi-threaded runtimes (e.g. `tokio::spawn`).

use noms_core::{Client, Notification};

fn assert_send<T: Send>(_: T) {}

#[test]
fn client_send_future_is_send() {
    let client = Client::default();
    let notification = Notification {
        title: None,
        message: String::new(),
        tags: vec![],
    };
    assert_send(client.send(&notification));
}
