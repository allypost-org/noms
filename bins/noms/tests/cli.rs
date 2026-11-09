#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_noms"))
        .env_clear()
        .env("PATH", std::env::var_os("PATH").unwrap_or_default())
        .env("HOME", "/tmp/noms-test-no-home")
        .env("XDG_CONFIG_HOME", "/tmp/noms-test-no-config")
        .current_dir(std::env::temp_dir())
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn tag_filter_does_not_contact_unselected_destinations() {
    let output = run(&[
        "send",
        "--service-url",
        "smtp://127.0.0.1:1?from=a@example.com&to=b@example.com#tag[]=private",
        "--message",
        "hello",
        "--tag",
        "public",
    ]);
    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Sending notification"),
        "filtered destination contacted: {stdout}"
    );
}

#[test]
fn invalid_destinations_never_print_credentials() {
    let output = run(&[
        "send",
        "--service-url",
        "smtp://alice:unique-password@localhost?to=to@example.com",
        "--message",
        "hello",
    ]);
    assert!(!output.status.success());
    let diagnostics = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !diagnostics.contains("unique-password"),
        "leaked credentials: {diagnostics}"
    );
}

#[test]
fn malformed_configuration_never_echoes_the_raw_url() {
    let output = run(&[
        "send",
        "--service-url",
        "smtp://alice:unique-password@localhost?from=a@example.com#timeout=not-a-duration",
        "--message",
        "hello",
    ]);
    assert!(!output.status.success());
    let diagnostics = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !diagnostics.contains("unique-password"),
        "leaked credentials: {diagnostics}"
    );
}

#[test]
fn cli_delivers_selected_mailboxes_and_keeps_message_bodies_out_of_trace_logs() {
    use std::{
        io::{BufRead, BufReader, Write},
        net::TcpListener,
        thread,
        time::Duration,
    };
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let receiver = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        socket.write_all(b"220 localhost ESMTP\r\n").unwrap();
        let mut input = BufReader::new(socket.try_clone().unwrap());
        let mut transcript = String::new();
        loop {
            let mut line = String::new();
            if input.read_line(&mut line).unwrap() == 0 {
                break;
            }
            transcript.push_str(&line);
            if line.starts_with("EHLO") {
                socket
                    .write_all(b"250-localhost\r\n250 8BITMIME\r\n")
                    .unwrap();
            } else if line.starts_with("DATA") {
                socket.write_all(b"354 Send message\r\n").unwrap();
                loop {
                    let mut line = String::new();
                    input.read_line(&mut line).unwrap();
                    if line == ".\r\n" {
                        break;
                    }
                    transcript.push_str(&line);
                }
                socket.write_all(b"250 Queued\r\n").unwrap();
                break;
            } else {
                socket.write_all(b"250 OK\r\n").unwrap();
            }
        }
        transcript
    });
    let destination = format!(
        "smtp://{address}?tls=none&from=sender@example.com&to=One <one@example.com>, Two <two@example.com>#tag[]=public&tag[]=urgent"
    );
    let output = run(&[
        "--log-level",
        "noms_core=trace",
        "send",
        "--service-url",
        &destination,
        "--service-url",
        "smtp://secret-user:secret-password@127.0.0.1:1",
        "--message",
        "private-message-body",
        "--tag",
        "public",
        "--tag",
        "urgent",
    ]);
    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let transcript = receiver.join().unwrap();
    for recipient in ["one@example.com", "two@example.com"] {
        assert!(
            transcript
                .to_ascii_lowercase()
                .contains(&format!("rcpt to:<{recipient}>"))
        );
    }
    assert!(transcript.contains("private-message-body"));
    let logs = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    for secret in ["private-message-body", "secret-password"] {
        assert!(!logs.contains(secret));
    }
}

#[test]
fn dry_run_selects_and_validates_destinations_without_connecting() {
    use std::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let destination = format!(
        "smtp://alice:unique-password@{}?from=from@example.com&to=to@example.com#tag[]=public&tag[]=urgent",
        listener.local_addr().unwrap()
    );
    let output = run(&[
        "send",
        "--dry-run",
        "--service-url",
        &destination,
        "--service-url",
        "smtp://invalid#tag[]=private",
        "--message",
        "private-body",
        "--tag",
        "public",
        "--tag",
        "urgent",
    ]);
    let diagnostics = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.status.success(), "dry run failed: {diagnostics}");
    assert!(diagnostics.contains("Would send notification to smtp://[redacted]"));
    assert!(diagnostics.contains("public, urgent"));
    assert!(!diagnostics.contains("private"));
    assert!(!diagnostics.contains("unique-password"));
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}

#[test]
fn dry_run_reports_invalid_selected_destinations() {
    let output = run(&[
        "send",
        "--dry-run",
        "--abort-on-error",
        "--service-url",
        "smtp://alice:unique-password@localhost?to=to@example.com",
        "--message",
        "hello",
    ]);
    assert_eq!(output.status.code(), Some(1));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("Would send"));
}

#[test]
fn dry_run_fails_when_no_destinations_match() {
    let output = run(&[
        "send",
        "--dry-run",
        "--service-url",
        "smtp://localhost?from=from@example.com&to=to@example.com#tag[]=private",
        "--message",
        "hello",
        "--tag",
        "public",
    ]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stdout).contains("No valid service URLs"));
}
