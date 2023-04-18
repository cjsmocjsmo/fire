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
    // for m in music_list {
    //     println!("{:?}", m);
    // }
    

    // let video_list: Vec<String> = media_lists.1;
    // for m in &video_list {
    //     if m.contains("Movies") {
    //         println!("{}", m);
    //     } else {
    //         println!("{}", m);
    //     }
    // }

    let images_list: Vec<String> = media_lists.2;
    for i in images_list {
        if i.contains("Posters2") {
            println!("{}", i);
        }
    }
    

    let mut result = false;

    if env_result {
        result = true;
    }

    result
}