// use std::fs;
pub mod fire_env_vars;
// pub mod fire_clean;

pub fn run_setup() -> bool {
    let mut env_result = false;
    let set_env_vars = fire_env_vars::read_config();
    if set_env_vars {
        print!("env vars are set");
        env_result = true;
    }

    let mut result = false;

    if env_result {
        result = true;
    }

    result
}