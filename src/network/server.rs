use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

pub struct MinecraftServer {
    listener: TcpListener
}

impl MinecraftServer {
    pub async fn new(addr: SocketAddr) -> Result<Self, std:io::Error> {
        let listener = TcpListener::bind(&addr).await?;
        Ok(Self {listener})
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Servidor de Minecraft en ejecución en {}", self.listener.local_addr()?);

        while let Ok((stream, _)) = self.listener.accept().await {
            tokio::spawn(Self::handle_client(stream));
        }

        Ok(())
    }
}