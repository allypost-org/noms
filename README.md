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

The reusable library currently lives in `crates/app-services`. Use a path dependency until publishing is configured. Email and Telegram are default features; select either independently with `default-features = false, features = ["email"]` or `["telegram"]`.

`Client::default()` uses the built-in registry. Add destinations with `add_url`, then call `send(&notification)` to get a report per selected destination. The client reuses validated senders and transports and bounds concurrent dispatch. It does not retry failed sends.

Custom providers implement `Service::prepare` and return a typed `PreparedService` implementation. Register them through `Registry::register` and pass the registry to `Client::new`. No JSON configuration downcasting or provider serialization is required. Telegram also supports a custom Bot API base and a caller-owned HTTP client.

Package naming and registry publishing are intentionally deferred.
