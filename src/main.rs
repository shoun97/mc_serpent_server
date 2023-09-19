use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está definida en el archivo .env");
    let api_key = env::var("API_KEY").expect("API_KEY no está definida en el archivo .env");

    println!("DATABASE_URL: {}", database_url);
    println!("API_KEY: {}", api_key);
}
