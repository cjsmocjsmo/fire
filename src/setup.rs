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


pub fn run_setup() -> bool {
    let paras = fire_env_vars::read_config();
    crate::setup::fire_env_vars::set_all_env_vars(paras);


    let media_lists = fire_walk_dirs::scan_all_sources();
    
    let images_list: Vec<String> = media_lists.2;

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    let mut index = 0;
    for i in images_list {
        index = index + 1;
        if i.contains("Posters2") {
            let tx = tx.clone();
            pool.execute(move || {
                let img_info = fire_process_movie_images::process_movie_posters(i.clone(), index);
            tx.send(img_info).expect("Could not send data");
            println!("{}", i.clone());
            });
            
        }
    };
    
    drop(tx);
    for t in rx.iter() {
        let ifo = t;
        println!("{}", ifo);
    }   

    true
}