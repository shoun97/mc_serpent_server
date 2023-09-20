use std::net::SocketAddr;
use tokio::net::TcpListener;
mod client_handler; 

pub struct MinecraftServer {
    listener: TcpListener
}

impl MinecraftServer {
    pub async fn new(addr: SocketAddr) -> Result<Self, std::io::Error> {
        let listener: TcpListener = TcpListener::bind(&addr).await?;
        Ok(Self {listener})
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Servidor de Minecraft en ejecución en {}", self.listener.local_addr()?);

        while let Ok((stream, _)) = self.listener.accept().await {
            tokio::spawn(client_handler::client_handler(stream)); // Usa client_handler::client_handler
        }
        
        Ok(())
    }
}