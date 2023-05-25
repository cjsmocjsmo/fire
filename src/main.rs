use std::time::Instant;
use dotenvy::dotenv;

pub mod setup;
pub mod server;
pub mod movies;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let start = Instant::now();
    dotenv().ok();
    let setup_result = setup::run_setup();
    let duration = start.elapsed();
    println!("\nSetup is Complete! \nStarting Server\n{:?} {}", duration, setup_result);
    if setup_result {
        let _start_server = crate::server::fire_server_main();
    };
    Ok(())
}
