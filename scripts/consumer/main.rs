use std::{
    error::Error,
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    sync::Arc,
    thread,
    time::Duration,
};

use noms::{Client, Notification, Registry, service::TelegramService};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

fn receive_email(listener: TcpListener) -> Result<String> {
    let (mut socket, _) = listener.accept()?;
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    socket.write_all(b"220 localhost ESMTP\r\n")?;
    let mut reader = BufReader::new(socket.try_clone()?);
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Err("SMTP disconnected before delivery".into());
        }
        if line.starts_with("EHLO") {
            socket.write_all(b"250-localhost\r\n250 8BITMIME\r\n")?;
        } else if line.starts_with("DATA") {
            socket.write_all(b"354 Send message\r\n")?;
            let mut message = String::new();
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line)? == 0 {
                    return Err("Incomplete SMTP message".into());
                }
                if line == ".\r\n" {
                    break;
                }
                message.push_str(&line);
            }
            socket.write_all(b"250 Queued\r\n")?;
            return Ok(message);
        } else {
            socket.write_all(b"250 OK\r\n")?;
        }
    }
}

fn receive_telegram(listener: TcpListener) -> Result<serde_json::Value> {
    let (mut socket, _) = listener.accept()?;
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut reader = BufReader::new(socket.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    assert!(request_line.starts_with("POST /bot123:consumer-token/sendMessage HTTP/1.1"));
    let mut length = None;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Err("Incomplete HTTP headers".into());
        }
        if line == "\r\n" {
            break;
        }
        if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
            length = Some(value.trim().parse::<usize>()?);
        }
    }
    let mut body = vec![0; length.ok_or("Missing request length")?];
    reader.read_exact(&mut body)?;
    socket.write_all(
        b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\nConnection: close\r\n\r\n{\"ok\":true}",
    )?;
    Ok(serde_json::from_slice(&body)?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let smtp = TcpListener::bind("127.0.0.1:0")?;
    let smtp_address = smtp.local_addr()?;
    let http = TcpListener::bind("127.0.0.1:0")?;
    let http_address = http.local_addr()?;
    let email = thread::spawn(move || receive_email(smtp));
    let telegram = thread::spawn(move || receive_telegram(http));
    let mut registry = Registry::default();
    registry.register(Arc::new(
        TelegramService::default().with_api_base(format!("http://{http_address}/").parse()?),
    ));
    let mut client = Client::new(registry);
    client.add_url(&format!("smtp://{smtp_address}?tls=none&from=sender@example.com&to=receiver@example.com#tag[]=consumer").parse()?)?;
    client.add_url(&"tgram://123:consumer-token/-100123#tag[]=consumer".parse()?)?;
    let reports = client
        .send(&Notification {
            title: Some("Consumer check".into()),
            message: "Delivered from an independent project.".into(),
            tags: vec!["consumer".into()],
        })
        .await;
    assert_eq!(reports.len(), 2);
    for report in reports {
        report.result?;
    }
    let message = email.join().map_err(|_| "SMTP receiver panicked")??;
    assert!(message.contains("Subject: Consumer check"));
    assert!(message.contains("Delivered from an independent project."));
    let message = telegram.join().map_err(|_| "HTTP receiver panicked")??;
    assert_eq!(message["text"], "Delivered from an independent project.");
    assert_eq!(message["chat_id"], -100_123);
    assert!(message.get("parse_mode").is_none());
    println!(
        "Packaged noms-core delivered email and Telegram notifications from an independent binary."
    );
    Ok(())
}
