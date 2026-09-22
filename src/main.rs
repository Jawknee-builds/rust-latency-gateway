use rust_latency_gateway::{http::{Method, Request}, upstream::MockUpstream, App};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let address = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = TcpListener::bind(&address)?;
    println!("rust-latency-gateway listening on http://{address}");
    let app = App::new(128, MockUpstream);
    for stream in listener.incoming() { if let Ok(stream) = stream { serve(stream, &app); } }
    Ok(())
}

fn serve<U: rust_latency_gateway::upstream::Upstream>(mut stream: TcpStream, app: &App<U>) {
    let mut buffer = [0; 4096];
    let Ok(size) = stream.read(&mut buffer) else { return };
    let line = String::from_utf8_lossy(&buffer[..size]).lines().next().unwrap_or("");
    let mut parts = line.split_whitespace();
    let method = match parts.next() { Some("GET") => Method::Get, _ => Method::Post };
    let path = parts.next().unwrap_or("/").to_string();
    let response = app.routes(Request { method, path });
    let wire = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\nConnection: close\r\n{}\r\n{}", response.status.0, response.body.len(), response.headers.iter().map(|(k,v)| format!("{k}: {v}\r\n")).collect::<String>(), response.body);
    let _ = stream.write_all(wire.as_bytes());
}
