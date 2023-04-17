// use std::fs;
// use std::env;

// use crate::setup::fire_walk_dirs::walk_posters2_dir;
pub mod fire_env_vars;
pub mod fire_walk_dirs;

pub fn run_setup() -> bool {
    let mut env_result = false;
    let isset_env_vars = fire_env_vars::read_config();
    if isset_env_vars {
        print!("env vars are set");
        env_result = true;
    }

    // let homedir = fire_walk_dirs::home_dir();
    // println!("this is home dir {}", homedir.clone());
    
    // let music_dir = homedir.clone() + "/Music";
    // let music_list = fire_walk_dirs::walk_music_dir_music(music_dir);

    // let video_dir = homedir.clone() + "/Videos";
    // let video_list = fire_walk_dirs::walk_video_dir(video_dir);
    
    // // walk FIRE_ADD_VIDEO_PATH
    // let add_vids = env::var("FIRE_ADD_VIDEO_PATH").expect("$FIRE_ADD_VIDEO_PATH is not set");
    // let nonE = String::from("NONE");
    // if add_vids != nonE {
    //     let vlist2 = fire_walk_dirs::walk_music_dir_music(add_vids);
    // }
    

    // // walk FIRE_ADD_MUSIC_PATH
    // let add_music = env::var("FIRE_ADD_MUSIC_PATH").expect("$FIRE_ADD_MUSIC_PATH is not set");
    // let mlist2 = fire_walk_dirs::walk_music_dir_music(add_music);

    // // scan posters2 
    // // scan mp3 folders for images

    let mut result = false;

    if env_result {
        result = true;
    }

    result
}