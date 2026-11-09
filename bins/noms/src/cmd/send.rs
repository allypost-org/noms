use noms_core::Client;

use crate::{
    config::run::send::SendConfig,
    logger::{p_err, p_info},
};

pub async fn run(send_config: SendConfig) -> super::CmdResult {
    let mut client = Client::default();
    let mut validation_errors = 0;
    let mut selected = Vec::new();
    for url in &send_config.service_urls {
        if !send_config
            .tags
            .iter()
            .all(|tag| url.settings.tags.contains(tag))
        {
            continue;
        }
        if let Err(error) = client.add_url(url) {
            p_err(&format!("Failed to validate {url}: {error}"));
            validation_errors += 1;
        } else {
            selected.push(url);
        }
    }
    if validation_errors > 0 && send_config.abort_on_error {
        return Err(("Failed to validate some service URLs.".into(), 1.into()));
    }
    if send_config.dry_run {
        if selected.is_empty() {
            return Err((
                "No valid service URLs match the requested tags.".into(),
                2.into(),
            ));
        }
        for url in selected {
            let tags = &url.settings.tags;
            let label = if tags.is_empty() {
                url.to_string()
            } else {
                format!("{url} ({})", tags.join(", "))
            };
            p_info(&format!("Would send notification to {label}"));
        }
        p_info("Dry run complete; no notifications sent.");
        return Ok(());
    }
    let abort_on_error = send_config.abort_on_error;
    let reports = client.send(&send_config.into_notification()).await;
    if reports.is_empty() {
        return Err((
            "No valid service URLs match the requested tags.".into(),
            2.into(),
        ));
    }
    let mut errors = 0;
    for report in &reports {
        match &report.result {
            Ok(()) => p_info(&format!(
                "Successfully sent notification to {}",
                report.destination
            )),
            Err(error) => {
                errors += 1;
                p_err(&format!(
                    "Failed to send notification to {}: {error}",
                    report.destination
                ));
            }
        }
    }
    if errors > 0 && abort_on_error {
        return Err(("Some notifications failed.".into(), 4.into()));
    }
    if errors == reports.len() {
        return Err(("Every notification failed.".into(), 5.into()));
    }
    p_info("Done");
    Ok(())
}
