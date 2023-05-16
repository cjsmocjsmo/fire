use std::time::Instant;
use dotenvy::dotenv;
use mongodb::Client;

pub mod setup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let start = Instant::now();
    dotenv().ok();
    let setup_result = setup::run_setup(client);
    let duration = start.elapsed();
    println!("\nSetup is Complete! {:?} {}", duration, setup_result);

    Ok(())
}
