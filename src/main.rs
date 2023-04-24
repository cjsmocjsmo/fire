use std::time::{Duration, Instant};

pub mod setup;

fn main() {
    let start = Instant::now();
    let setup_result = setup::run_setup();
    let duration = start.elapsed();
    println!("\nSetup is Complete! {:?} {}", duration, setup_result);
}
