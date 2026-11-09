use std::net::TcpListener;

fn main() {
    let server = TcpListener::bind("127.0.0.1:8787")
        .expect("Não foi possível iniciar o servidor");

    println!("Roblox Cloud PC iniciado!");
    println!("Endereço local: http://127.0.0.1:8787");

    for connection in server.incoming() {
        match connection {
            Ok(stream) => {
                drop(stream);
                println!("Conexão recebida!");
            }
            Err(error) => eprintln!("Erro: {error}"),
        }
    }
}
