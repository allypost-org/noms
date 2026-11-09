#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn request(address: std::net::SocketAddr, path: &str, auth: Option<&str>) -> String {
    let mut socket = TcpStream::connect(address).unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    write!(
        socket,
        "GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n"
    )
    .unwrap();
    if let Some(token) = auth {
        write!(socket, "Authorization: Bearer {token}\r\n").unwrap();
    }
    write!(socket, "\r\n").unwrap();
    let mut response = String::new();
    socket.read_to_string(&mut response).unwrap();
    response
}

#[test]
fn server_health_date_and_jwt_sessions_work_after_upgrading() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let child = Command::new(env!("CARGO_BIN_EXE_noms-server"))
        .env_clear()
        .env("PATH", std::env::var_os("PATH").unwrap_or_default())
        .env("HOME", "/tmp/noms-test-no-home")
        .env("XDG_CONFIG_HOME", "/tmp/noms-test-no-config")
        .current_dir(std::env::temp_dir())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args([
            "server",
            "--host",
            "127.0.0.1",
            "--port",
            &address.port().to_string(),
            "--no-run-admin",
            "--jwt-secret",
            "test-signing-key-for-boundary-test",
        ])
        .spawn()
        .unwrap();
    let mut server = Server(child);
    let start = Instant::now();
    loop {
        if TcpStream::connect(address).is_ok() {
            break;
        }
        assert!(
            server.0.try_wait().unwrap().is_none(),
            "server exited during startup"
        );
        assert!(
            start.elapsed() < Duration::from_secs(5),
            "server did not start"
        );
        thread::sleep(Duration::from_millis(10));
    }
    let health = request(address, "/healthz", None);
    assert!(health.starts_with("HTTP/1.1 200"));
    assert!(health.ends_with("OK"));
    let date = health
        .lines()
        .find_map(|line| line.strip_prefix("date: "))
        .unwrap();
    assert!(date.ends_with("GMT"));
    assert!(jiff::fmt::rfc2822::parse(date).is_ok());
    let session = request(address, "/plus-plus", None);
    assert!(session.starts_with("HTTP/1.1 200"));
    let token = session
        .split_whitespace()
        .map(|part| part.trim_matches('"'))
        .find(|part| part.split('.').count() == 3)
        .unwrap();
    let verified = jsonwebtoken::decode::<serde_json::Value>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(b"test-signing-key-for-boundary-test"),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512),
    )
    .unwrap();
    assert_ne!(verified.claims["sub"].as_str().unwrap(), "");
    let expires = verified.claims["exp"].as_i64().unwrap();
    let now = jiff::Timestamp::now().as_second();
    assert!((now..=now + 300).contains(&expires));
    let authenticated = request(address, "/plus-plus", Some(token));
    assert!(authenticated.starts_with("HTTP/1.1 200"));
}

#[test]
fn server_help_never_displays_environment_credentials() {
    let output = Command::new(env!("CARGO_BIN_EXE_noms-server"))
        .current_dir(std::env::temp_dir())
        .env("NOMS_SERVER_API_JWT_SECRET", "unique-signing-secret")
        .env(
            "NOMS_DATABASE_URL",
            "libsql://example.com?token=unique-database-token",
        )
        .args(["server", "--help"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let help = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    for secret in ["unique-signing-secret", "unique-database-token"] {
        assert!(!help.contains(secret));
    }
}

#[test]
fn invalid_database_configuration_does_not_echo_credentials() {
    let output = Command::new(env!("CARGO_BIN_EXE_noms-server"))
        .current_dir(std::env::temp_dir())
        .args([
            "server",
            "--database-url",
            "ftp://alice:unique-database-password@example.com",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let diagnostic = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!diagnostic.contains("unique-database-password"));
}
