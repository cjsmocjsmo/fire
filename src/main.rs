use std::time::Instant;
use dotenvy::dotenv;

pub mod setup;

fn main() {
    let start = Instant::now();
    dotenv().ok();
    let setup_result = setup::run_setup();
    let duration = start.elapsed();
    println!("\nSetup is Complete! {:?} {}", duration, setup_result);
}
