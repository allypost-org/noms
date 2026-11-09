# NOMS

Send notifications through service URLs, from the CLI or a Rust application.

## Development

```sh
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo fmt --all --check
```

Tests use executable, public library, and local HTTP/SMTP/TLS boundaries. They do not contact real notification services or require credentials.

## Sending

```sh
cargo run -p noms -- send \
  --service-url 'smtp://user:password@smtp.example.com:587?from=sender@example.com&to=recipient@example.com#tag[]=alerts' \
  --message 'Something happened.' --tag alerts
```

Repeat `--service-url` for multiple destinations. Commas inside a URL are preserved for mailbox lists. Each requested tag must match a destination; no tags selects all destinations. `--abort-on-error` prevents sending after validation errors and returns a failure status for any delivery error. Once delivery begins, other destinations finish independently.

Add `--dry-run` (or set `NOMS_SEND_DRY_RUN=true`) to validate and list selected destinations without connecting or sending. Credentials stay redacted. Tag filtering and validation error handling are the same as a normal send; use `--abort-on-error` to fail on any invalid selected destination. A dry run does not check connectivity or delivery acceptance.

SMTP requires STARTTLS by default. `smtps://` uses implicit TLS. Plaintext requires an explicit `?tls=none` opt-in. Email supports `from`, `to`, `cc`, `bcc`, and `reply-to` query parameters. The default body format is plain text; `#format=html` selects HTML. BCC addresses are included in the SMTP envelope and excluded from message headers.

Telegram URLs accept signed chat IDs and channel usernames, for example `tgram://BOT_TOKEN/-1001234567890/@channel_name`. Messages default to plain text. Use `#format=html`, `#format=markdown`, or `?parse_mode=MarkdownV2` for explicitly formatted input; callers are responsible for escaping that input. Boolean parameters such as `disable_notification=false` honor their values.

Common settings go in the URL fragment: `#timeout=4s&verify=true&tag[]=alerts`. Jiff parses durations, including `20ms`, `4 seconds`, and ISO 8601 durations. Timeouts must be positive. Certificate verification is enabled by default; `verify=false` explicitly disables it.

URLs contain credentials. Diagnostic `Display`/`Debug` representations are redacted; configuration serialization and `original_url` deliberately retain the complete URL. Keep serialized configuration private.

## Library

Use [`noms-core`](crates/noms-core/README.md) to send notifications directly from
another Rust project. It requires Rust 1.88+ and a Tokio runtime. For example:

```toml
[dependencies]
noms = { package = "noms-core", version = "0.1", default-features = false, features = ["telegram"] }
```

Import `noms::{Client, Notification}` when using that alias. Both email and Telegram
are enabled by default; provider features can be selected independently. Clients
reuse prepared transports and return a report for each selected destination.
See the library README for a complete example, error handling and configuration.

Until the initial publication, use a path dependency on `crates/noms-core` or a Git
dependency on this repository. CI builds and verifies the package; core release
tags publish it once Trusted Publishing is configured. The CLI and server are
currently marked private for registry publishing.

## Releases and package verification

See [RELEASING.md](RELEASING.md) for CI, the initial crates.io publication, Trusted
Publishing setup and release tags (`noms-core-v0.1.0`).

```sh
# Verify an uncommitted package from a standalone binary in ignored ./dev:
scripts/check-core-package --allow-dirty
```
