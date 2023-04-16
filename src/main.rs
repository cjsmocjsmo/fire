pub mod setup;

fn main() {
    let setup_result = setup::run_setup();
    println!("\nSetup is Complete! {}", setup_result);
}
