// use std::fs;
pub mod fire_env_vars;
pub mod fire_walk_dirs;

pub fn run_setup() -> bool {
    let mut env_result = false;
    let isset_env_vars = fire_env_vars::read_config();
    if isset_env_vars {
        print!("env vars are set");
        env_result = true;
    }

    let homedir = fire_walk_dirs::home_dir();
    println!("{}", homedir);

    let mut result = false;

    if env_result {
        result = true;
    }

    result
}