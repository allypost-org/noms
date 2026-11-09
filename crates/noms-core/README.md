# noms-core

Send notifications to email and Telegram destinations from an async Rust application.

## Install

Requires Rust 1.88 or newer and a Tokio runtime with networking and timers enabled.
Both providers are enabled by default. Select only what your application uses:

```toml
[dependencies]
noms = { package = "noms-core", version = "0.1", default-features = false, features = ["telegram"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net", "time"] }
```

The `noms` dependency name is a local alias; the registry package is `noms-core`.

## Send a notification

```rust,no_run
use noms::{Client, Notification};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::default();
    let destination = std::env::var("NOTIFICATION_URL")?;
    client.add_url(&destination.parse()?)?;

    // Keep this client in application state and reuse it for subsequent sends.
    let reports = client.send(&Notification {
        title: Some("Job finished".into()),
        message: "Your export is ready.".into(),
        tags: vec![],
    }).await;

    for report in reports {
        if let Err(error) = report.result {
            eprintln!("Notification to {} failed: {error}", report.destination);
        }
    }
    Ok(())
}
```

`add_url` validates configuration without contacting the destination. `send` waits
for all selected deliveries and returns one report per destination in completion
order. Check each result: returning from `send` does not imply every delivery
succeeded. No matching destinations returns an empty vector. One destination's
failure does not cancel others, and there are no automatic retries. Telegram
URLs listing multiple chat IDs send sequentially and stop at the first failed
chat; some chats may already have received the notification. Retrying can create
duplicates. The client bounds concurrent destinations to 16 and reuses prepared
HTTP clients and SMTP transports. The caller owns the Tokio runtime and logging
subscriber; the library never installs either globally.

## Destinations

- Telegram: `tgram://BOT_TOKEN/-1001234567890/@channel_name`
- Email: `smtp://user:password@smtp.example.com:587?from=sender@example.com&to=recipient@example.com`
- Implicit SMTP TLS: `smtps://user:password@smtp.example.com?from=sender@example.com&to=recipient@example.com`

Percent-encode reserved characters in credentials and query values. Email supports
`cc`, `bcc`, and `reply-to`; BCC addresses are envelope recipients and are excluded
from message headers. Use indexed query values for multiple addresses, such as
`to[0]=one@example.com&to[1]=two@example.com`, or a mailbox list in one value.

Common settings go after `#`:

```text
tgram://BOT_TOKEN/123456#timeout=4s&tag[]=alerts&tag[]=urgent
```

A notification is selected only when all of its tags occur in the destination's
tags. Empty notification tags select every configured destination. Tags are
routing labels; they are not sent to the provider.

Bodies default to plain text. Use `#format=html` to send HTML. Telegram additionally
supports `#format=markdown` (MarkdownV2) and `?parse_mode=HTML`, `Markdown`, or
`MarkdownV2`; callers must escape explicitly formatted input. Telegram sends the
message body; its title is currently ignored. Email uses the title as the subject.

Timeouts default to four seconds. Jiff parses positive durations such as `20ms`,
`4 seconds`, and `PT4S`. HTTP timeouts cover connection and response reading;
SMTP timeouts apply to transport operations. A multi-chat Telegram destination
may therefore take longer than one timeout in total.

SMTP requires STARTTLS by default; plaintext is an explicit `?tls=none` opt-in.
Certificates are verified by default. The fragment setting `verify=false` disables
verification and should be reserved for environments where that is intentional.

URLs contain secrets. `Display`, `Debug`, built-in logs, and built-in error messages
redact credentials. `ServiceUrl::original_url` and configuration serialization
retain complete URLs; keep those values private. Notification bodies are not
logged by the library.

## Custom providers

Implement `service::Service::prepare` and return your own typed
`service::PreparedService`. Register the provider with `Registry::register`, then
construct `Client::new(registry)`. `Registry::default()` includes enabled built-in
providers; `Registry::new()` starts empty. Registering a provider with an existing
name replaces it. Provider errors and diagnostic implementations are responsible
for protecting their own credentials.

`service::TelegramService` supports a self-hosted Bot API base URL and a
caller-owned HTTP client. An injected client controls TLS, proxy and redirect
policy; per-request deadlines still apply.

## Compatibility and development

The library supports Rust 1.88+. CI builds the core at that minimum; workspace
tests and linting use the pinned development toolchain. Public API changes may
occur between `0.x` minor releases. Patch releases retain the public API.

See the [repository](https://github.com/allypost-org/noms) for CLI usage,
contribution checks, and the release process. The crate is MIT licensed.
