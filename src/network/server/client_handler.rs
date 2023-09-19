use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn client_handler(mut stream: TcpStream) {
    let mut buf = vec![0; 1024];

    match stream.read(&mut buf).await {
        Ok(n) => {
            let client_data = String::from_utf8_lossy(&buf[..n]);
            println!("Req: {}", client_data);

            let response = format!("Eco: {}", client_data);

            if let Err(e) = stream.write_all(response.as_bytes()).await {
                eprintln!("Error al escribir en el cliente: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error al leer datos del cliente: {}", e);
        }
    }
}
