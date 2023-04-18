// use std::fs;
// use std::env;

// use crate::setup::fire_walk_dirs::walk_posters2_dir;
pub mod fire_env_vars;
pub mod fire_walk_dirs;

pub fn run_setup() -> bool {
    let mut env_result = false;
    let isset_env_vars = fire_env_vars::read_config();
    if isset_env_vars {
        print!("env vars are set\n\n");
        env_result = true;
    }

    let media_lists = fire_walk_dirs::scan_all_sources();

    // let music_list: Vec<String> = media_lists.0;
    // println!("{:?}", music_list);

    let video_list: Vec<String> = media_lists.1;
    for m in &video_list {
        if m.contains("TVShows") {
            println!("{}", m);
        }
    }
    // println!("{:?}", video_list);

    // let images_list: Vec<String> = media_lists.2;
    // println!("{:?}", images_list);

    let mut result = false;

    if env_result {
        result = true;
    }

    result
}