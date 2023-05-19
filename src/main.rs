use std::time::Instant;
use dotenvy::dotenv;

pub mod setup;

fn main() {
    // let client = Client::with_uri_str("mongodb://localhost:27017/fire").await?;
    // let client = Client::with_uri_str("mongodb://db:27017/firedb").await?;
    
    let start = Instant::now();
    dotenv().ok();
    let setup_result = setup::run_setup();
    let duration = start.elapsed();
    println!("\nSetup is Complete! {:?} {}", duration, setup_result);

    
}
