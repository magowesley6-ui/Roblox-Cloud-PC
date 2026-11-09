use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8787")
        .expect("Erro ao iniciar servidor");

    println!("Roblox Cloud PC iniciado!");
    println!("Acesse http://127.0.0.1:8787");

    for connection in listener.incoming() {
        if let Ok(mut stream) = connection {
            let mut buffer = [0; 4096];

            if stream.read(&mut buffer).is_err() {
                continue;
            }

            let request = String::from_utf8_lossy(&buffer);
            let path = request.split_whitespace().nth(1).unwrap_or("/");

            let (status, content_type, body) = match path {
                "/health" => (
                    "200 OK",
                    "application/json",
                    r#"{"status":"online","app":"Roblox Cloud PC"}"#.to_string(),
                ),
                "/" | "/index.html" => {
                    match fs::read_to_string("web/index.html") {
                        Ok(html) => ("200 OK", "text/html; charset=utf-8", html),
                        Err(_) => (
                            "500 Internal Server Error",
                            "text/plain",
                            "Painel nao encontrado".to_string(),
                        ),
                    }
                }
                _ => (
                    "404 Not Found",
                    "text/plain",
                    "Pagina nao encontrada".to_string(),
                ),
            };

            let response = format!(
                "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                status,
                content_type,
                body.len(),
                body
            );

            let _ = stream.write_all(response.as_bytes());
        }
    }
}
