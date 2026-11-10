# Changelog

## 0.1.2

- `Client::send` now returns a `Send` future, so it can be used with `tokio::spawn`
  and other multi-threaded executors.

## 0.1.1

- Release through GitHub Actions Trusted Publishing.
- No notification API or behavior changes.

## 0.1.0

- Async notification client with reusable email and Telegram transports.
- Tag-based destination selection and per-destination delivery reports.
- Extensible provider registry with typed prepared senders.
- Secure TLS defaults, request timeouts, and redacted diagnostics.
