// use std::fs;
// use std::env;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
// use crate::setup::fire_walk_dirs::walk_posters2_dir;
pub mod fire_env_vars;
pub mod fire_walk_dirs;
pub mod fire_process_movie_images;
pub mod fire_image;
pub mod fire_misc;
pub mod fire_split;
pub mod fire_process_music_images;

fn run_img_threads(alist: Vec<String>) {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    let mut img_index = 0;
    for i in images_list {
        img_index = img_index + 1;
        if i.contains("Posters2") {
            let tx = tx.clone();
            pool.execute(move || {
                let img_info = fire_process_movie_images::process_movie_posters(i.clone(), img_index);
                tx.send(img_info).expect("Could not send data");
                println!("{}", i.clone());
            });
            
        } else if i.contains("Music") {
            let tx = tx.clone();
            pool.execute(move || {
                let img_info = fire_process_music_images::process_music_images(i.clone(), img_index);
                tx.send(img_info).expect("Could not send data");
            });
        }
    };
    
    drop(tx);
    for t in rx.iter() {
        let ifo = t;
        println!("{}", ifo);
    }   
}


pub fn run_setup() -> bool {
    let paramaters = fire_env_vars::read_config();

    crate::setup::fire_env_vars::set_all_env_vars(paramaters);

    let media_lists = fire_walk_dirs::scan_all_sources();

    let music_list = media_lists.0;
    for m in music_list {
        println!("{}\n", m)
    }

    let video_list = media_lists.1;
    for v in video_list {
        println!("{}\n", v)
    }
    
    let images_list: Vec<String> = media_lists.2;

    run_img_threads(images_list);

    true
}