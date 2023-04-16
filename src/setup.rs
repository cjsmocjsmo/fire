// use std::fs;
pub mod fire_env_vars;
pub mod fire_clean;

pub fn run_setup() -> i32 {
    // let cwd = fire_env_vars::get_current_working_dir();
    // let fire_dir = cwd.clone().to_string() + "/fire_dir";
    // fs::create_dir_all(fire_dir).unwrap();

    // let thumb_dir = cwd.clone().to_string() + "/fire_dir/thumbnails";
    // fs::create_dir_all(thumb_dir).unwrap();

    // let nfo_dir = cwd.clone().to_string() + "/fire_dir/nfos";
    // fs::create_dir_all(nfo_dir).unwrap();
    
    
    // get current working dir
    // set env vars using current working dir and 
    // clean dirs if they exist, create dirs/files as needed 
    // 
    //

    let set_env_vars = fire_env_vars::read_config();
    if set_env_vars {
        print!("env vars are set")
    }

    return 1
}