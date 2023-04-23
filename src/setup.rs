// use std::fs;
// use std::env;

// use crate::setup::fire_walk_dirs::walk_posters2_dir;
pub mod fire_env_vars;
pub mod fire_walk_dirs;
pub mod fire_process_movie_images;
pub mod fire_image;
pub mod fire_misc;
pub mod fire_split;


pub fn run_setup() -> bool {
    let paras = fire_env_vars::read_config();
    crate::setup::fire_env_vars::set_all_env_vars(paras);


    let media_lists = fire_walk_dirs::scan_all_sources();
    

    let images_list: Vec<String> = media_lists.2;
    let mut index = 0;
    for i in images_list {
        index = index + 1;
        if i.contains("Posters2") {
            // fire_process_movie_images::process_movie_posters(i.clone(), index);
            println!("{}", i.clone());
        }
    };
    

    true
}