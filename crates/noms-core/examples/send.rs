use noms_core::{Client, Notification};

/// Run with `NOTIFICATION_URL` set to a real email or Telegram destination.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::default();
    client.add_url(&std::env::var("NOTIFICATION_URL")?.parse()?)?;
    let reports = client
        .send(&Notification {
            title: Some("Notification example".into()),
            message: std::env::var("NOTIFICATION_MESSAGE")
                .unwrap_or_else(|_| "Hello from noms-core.".into()),
            tags: vec![],
        })
        .await;
    let mut failed = false;
    for report in reports {
        match report.result {
            Ok(()) => println!("Delivered to {}", report.destination),
            Err(error) => {
                failed = true;
                eprintln!("Failed to deliver to {}: {error}", report.destination);
            }
        }
    }
    if failed {
        return Err("One or more notifications failed".into());
    }
    Ok(())
}
