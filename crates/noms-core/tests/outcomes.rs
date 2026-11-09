#![allow(clippy::unwrap_used, clippy::expect_used)]

use noms_core::ServiceUrl;
#[cfg(any(feature = "email", feature = "telegram"))]
use noms_core::{Notification, service::AVAILABLE_SERVICES};
use std::time::Duration;
#[cfg(feature = "telegram")]
use std::time::Instant;
#[cfg(any(feature = "email", feature = "telegram"))]
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

#[cfg(any(feature = "email", feature = "telegram"))]
fn notification() -> Notification {
    Notification {
        title: Some("A title".into()),
        message: "Hello, world! (plain text)".into(),
        tags: vec![],
    }
}

#[cfg(any(feature = "email", feature = "telegram"))]
fn prepare(raw: &str) -> Result<noms_core::service::AssociatedService, String> {
    let url: ServiceUrl = raw.parse().map_err(|e| format!("{e}"))?;
    AVAILABLE_SERVICES
        .iter()
        .find_map(|s| url.associate_if_valid(s.as_ref()))
        .ok_or_else(|| "unsupported service".to_string())?
        .map_err(|e| e.to_string())
}

#[test]
fn diagnostics_hide_credentials_but_configuration_round_trips() {
    let raw = "mailto://alice:super-secret@example.com?from=alice@example.com&to=bob@example.com";
    let url: ServiceUrl = raw.parse().unwrap();
    for output in [url.to_string(), format!("{url:?}")] {
        assert!(
            !output.contains("super-secret"),
            "credential in diagnostic: {output}"
        );
    }
    let encoded = serde_json::to_string(&url).unwrap();
    let restored: ServiceUrl = serde_json::from_str(&encoded).unwrap();
    assert_eq!(serde_json::to_string(&restored).unwrap(), encoded);
    assert!(encoded.contains("super-secret"));
}

#[test]
fn malformed_url_diagnostics_do_not_echo_input() {
    let err = "secret-input-without-scheme"
        .parse::<ServiceUrl>()
        .unwrap_err();
    assert!(!format!("{err} {err:?}").contains("secret-input"));
}

#[test]
fn duration_settings_round_trip_and_reject_overflow() {
    let url: ServiceUrl = "tgram://123:abc/42#timeout=4%20seconds".parse().unwrap();
    let encoded = serde_json::to_string(&url.settings).unwrap();
    let decoded: noms_core::service::service_url::ServiceSettings =
        serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded.connection_timeout, Duration::from_secs(4));
    for invalid in ["18446744073709551615 months", "-1s", "0s"] {
        assert!(
            format!("tgram://123:abc/42#timeout={invalid}")
                .parse::<ServiceUrl>()
                .is_err()
        );
    }
    let url: ServiceUrl = "tgram://123:abc/42#timeout=20ms".parse().unwrap();
    assert_eq!(url.settings.connection_timeout, Duration::from_millis(20));
}

#[cfg(feature = "telegram")]
#[tokio::test]
async fn requests_have_a_deadline_after_connection() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buf = [0; 1024];
        let _ = stream.read(&mut buf);
        thread::sleep(Duration::from_millis(250));
        let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK");
    });
    let url: ServiceUrl = "tgram://123:abc/42#timeout=20ms".parse().unwrap();
    let start = Instant::now();
    let result = url
        .as_reqwest_client()
        .unwrap()
        .get(format!("http://{address}"))
        .send()
        .await;
    server.join().unwrap();
    assert!(result.is_err(), "stalled server should time out");
    assert!(start.elapsed() < Duration::from_secs(2));
}

#[cfg(feature = "telegram")]
#[test]
fn telegram_accepts_group_and_channel_destinations_and_rejects_bad_tokens() {
    assert!(prepare("tgram://123:abc_DEF-123/-1001234567890/@my_channel").is_ok());
    assert!(prepare("tgram://123:abc!invalid/42").is_err());
    assert!(prepare("tgram://123:abc/42?disable_notification=maybe").is_err());
}

#[cfg(feature = "email")]
#[tokio::test]
async fn smtp_delivery_honors_envelope_headers_and_plain_text() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || smtp_receive(&listener));
    let sender = prepare(&format!("smtp://{address}?tls=none&from=from@example.com&to=to@example.com&cc=cc@example.com&bcc=hidden@example.com&reply-to=reply@example.com")).unwrap();
    sender.send(&notification()).await.unwrap();
    let (commands, message) = server.join().unwrap();
    for recipient in ["to@example.com", "cc@example.com", "hidden@example.com"] {
        assert!(
            commands
                .to_lowercase()
                .contains(&format!("rcpt to:<{recipient}>")),
            "missing envelope recipient: {commands}"
        );
    }
    assert!(message.contains("Cc: cc@example.com"));
    assert!(message.contains("Reply-To: reply@example.com"));
    assert!(!message.contains("hidden@example.com"));
    assert!(message.contains("Content-Type: text/plain"));
    assert!(message.contains("Hello, world! (plain text)"));
}

#[cfg(feature = "email")]
fn smtp_receive(listener: &TcpListener) -> (String, String) {
    use std::io::{BufRead, BufReader};
    let (mut stream, _) = listener.accept().unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    stream.write_all(b"220 localhost ESMTP\r\n").unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut commands = String::new();
    let mut message = String::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap() == 0 {
            break;
        }
        commands.push_str(&line);
        if line.starts_with("EHLO") {
            stream
                .write_all(b"250-localhost\r\n250 8BITMIME\r\n")
                .unwrap();
        } else if line.starts_with("DATA") {
            stream.write_all(b"354 Send message\r\n").unwrap();
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line == ".\r\n" {
                    break;
                }
                message.push_str(&line);
            }
            stream.write_all(b"250 Queued\r\n").unwrap();
            break;
        } else {
            stream.write_all(b"250 OK\r\n").unwrap();
        }
    }
    (commands, message)
}

#[cfg(feature = "telegram")]
#[tokio::test]
async fn telegram_sends_plain_messages_and_honors_format_and_false_flags() {
    use noms_core::service::TelegramService;
    use std::sync::Arc;
    for suffix in [
        "?disable_notification=false&protect_content=false",
        "#format=html",
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base: url::Url = format!("http://{}/", listener.local_addr().unwrap())
            .parse()
            .unwrap();
        let receiver = thread::spawn(move || receive_http(&listener, b"{\"ok\":true}"));
        let url: ServiceUrl = format!("tgram://123:abc_DEF/-100123{suffix}")
            .parse()
            .unwrap();
        let service = Arc::new(TelegramService::default().with_api_base(base));
        let sender = url.associate_if_valid(service.as_ref()).unwrap().unwrap();
        sender.send(&notification()).await.unwrap();
        let body = receiver.join().unwrap();
        assert_eq!(body["chat_id"], -100_123);
        assert_eq!(body["text"], "Hello, world! (plain text)");
        assert_eq!(body["disable_notification"], false);
        assert_eq!(body["protect_content"], false);
        if suffix.contains("html") {
            assert_eq!(body["parse_mode"], "HTML");
        } else {
            assert!(body.get("parse_mode").is_none());
        }
    }
}

#[cfg(feature = "telegram")]
fn receive_http(listener: &TcpListener, response: &[u8]) -> serde_json::Value {
    use std::io::{BufRead, BufReader};
    let (mut stream, _) = listener.accept().unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut length = 0;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if line == "\r\n" {
            break;
        }
        if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
            length = value.trim().parse::<usize>().unwrap();
        }
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body).unwrap();
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        response.len()
    )
    .unwrap();
    stream.write_all(response).unwrap();
    serde_json::from_slice(&body).unwrap()
}

#[cfg(feature = "email")]
#[tokio::test]
async fn smtp_requires_starttls_by_default() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        use std::io::{BufRead, BufReader};
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        stream.write_all(b"220 localhost\r\n").unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut command = String::new();
        reader.read_line(&mut command).unwrap();
        stream.write_all(b"250 localhost\r\n").unwrap();
        let mut remaining = String::new();
        let _ = reader.read_to_string(&mut remaining);
        remaining
    });
    let sender = prepare(&format!(
        "smtp://{address}?from=from@example.com&to=to@example.com#timeout=100ms"
    ))
    .unwrap();
    assert!(sender.send(&notification()).await.is_err());
    let commands = server.join().unwrap();
    assert!(!commands.contains("DATA"));
    assert!(!commands.contains("Hello"));
}

#[cfg(feature = "telegram")]
#[tokio::test]
async fn self_signed_servers_are_rejected_unless_verification_is_explicitly_disabled() {
    use std::sync::Arc;
    for verify in [true, false] {
        let certified = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let config = rustls::ServerConfig::builder_with_provider(Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(
            vec![certified.cert.der().clone()],
            rustls::pki_types::PrivatePkcs8KeyDer::from(certified.signing_key.serialize_der())
                .into(),
        )
        .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let connection = rustls::ServerConnection::new(Arc::new(config)).unwrap();
            let mut tls = rustls::StreamOwned::new(connection, stream);
            let mut request = [0; 4096];
            if tls.read(&mut request).is_ok() {
                let _ = tls.write_all(
                    b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK",
                );
            }
        });
        let settings: ServiceUrl = format!("tgram://123:abc/42#verify={verify}&timeout=1s")
            .parse()
            .unwrap();
        let result = settings
            .as_reqwest_client()
            .unwrap()
            .get(format!("https://localhost:{}", address.port()))
            .send()
            .await;
        server.join().unwrap();
        assert_eq!(result.is_ok(), !verify);
    }
}

#[cfg(feature = "telegram")]
#[tokio::test]
async fn telegram_request_errors_do_not_expose_bot_tokens() {
    use std::sync::Arc;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let service = Arc::new(
        noms_core::service::TelegramService::default()
            .with_api_base(format!("http://{address}/").parse().unwrap()),
    );
    let url: ServiceUrl = "tgram://123:unique_secret_token/42".parse().unwrap();
    let sender = url.associate_if_valid(service.as_ref()).unwrap().unwrap();
    let error = sender.send(&notification()).await.unwrap_err();
    let diagnostic = format!("{error} {error:?}");
    assert!(!diagnostic.contains("unique_secret_token"));
}

#[cfg(feature = "telegram")]
#[tokio::test]
async fn malformed_telegram_responses_do_not_echo_reflected_credentials() {
    use std::sync::Arc;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let receiver =
        thread::spawn(move || receive_http(&listener, b"{\"ok\":\"123:unique_secret_token\"}"));
    let service = Arc::new(
        noms_core::service::TelegramService::default()
            .with_api_base(format!("http://{address}/").parse().unwrap()),
    );
    let url: ServiceUrl = "tgram://123:unique_secret_token/42".parse().unwrap();
    let sender = url.associate_if_valid(service.as_ref()).unwrap().unwrap();
    let error = sender.send(&notification()).await.unwrap_err();
    receiver.join().unwrap();
    let diagnostic = format!("{error} {error:?}");
    assert!(
        !diagnostic.contains("unique_secret_token"),
        "leaked credential: {diagnostic}"
    );
}
