use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use mc_serpent_server::network::server::MinecraftServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let port_str = env::var("PORT").expect("PORT no está definida en el archivo .env");
    let port: u16 = port_str.parse().expect("No se pudo convertir PORT a u16");
    println!("Server Running in PORT: {}", port);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let mut server = MinecraftServer::new(addr).await?;
    server.start().await?;
    Ok(())
}
