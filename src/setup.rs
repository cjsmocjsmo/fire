use std::env;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
// use serde::{Serialize, Deserialize};
 
pub mod fire_create_dirs;
pub mod fire_image;
pub mod fire_misc;
pub mod fire_mp3_info;
pub mod fire_process_movie_images;
pub mod fire_process_music;
pub mod fire_process_music_images;
pub mod fire_utils;
pub mod fire_walk_dirs;
pub mod fire_process_tvshows;

fn run_music_threads(alist: Vec<String>) {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    let mut index = 0;
    let mut page = 1;
    let mut page_count = 0;

    let ofs = env::var("FIRE_PAGINATION").unwrap();
    let offset: u32 = ofs.trim().parse().expect("offset conversion failed");

    for a in alist {
        index = index + 1;
        if page_count < offset {
            page_count = page_count + 1;
            page = page;
        } else {
            page_count = 1;
            page = page + 1;
        }
        let tx = tx.clone();
        pool.execute(move || {
            let mfi = crate::setup::fire_process_music::process_mp3s(
                a.clone(),
                index.to_string(),
                page.to_string(),
            );
            tx.send(mfi).expect("Could not send data");
        });
    }

    drop(tx);
    for t in rx.iter() {
        // Insert this into db
        let ifo = t;
        println!("{:#?}", ifo);
    }
}

fn run_video_img_threads(alist: Vec<String>) {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    let mut img_index = 0;
    for i in alist {
        img_index = img_index + 1;
        if i.contains("MovPosters") {
            let tx = tx.clone();
            pool.execute(move || {
                let img_info =
                    fire_process_movie_images::process_movie_posters(i.clone(), img_index);
                tx.send(img_info).expect("Could not send data");
                println!("{}", i.clone());
            });
        }
    }

    drop(tx);
    for t in rx.iter() {
        // Insert this into db
        let ifo = t;
        println!("{:#?}", ifo);
    }
}

fn run_music_img_threads(alist: Vec<String>) {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    let mut img_index = 0;
    for i in alist {
        img_index = img_index + 1;
        if i.contains("Music") {
            let tx = tx.clone();
            pool.execute(move || {
                let img_info =
                    fire_process_music_images::process_music_images(i.clone(), img_index);
                tx.send(img_info).expect("Could not send data");
            });
        }
    }

    drop(tx);
    for t in rx.iter() {
        // Insert this into db
        let ifo = t;
        println!("{:?}", ifo);
    }
}

pub fn run_setup() -> bool {
    // let paramaters = fire_env_vars::read_config();

    crate::setup::fire_create_dirs::create_dirs();

    let scan_home_dir = 
        env::var("FIRE_SCAN_HOME")
        .expect("$FIRE_SCAN_HOME is not set");

    if scan_home_dir == "yes" {
        let media_lists = fire_walk_dirs::scan_all_sources();

        run_music_threads(media_lists.0.clone());
        run_music_img_threads(media_lists.2.clone());
        run_video_img_threads(media_lists.2.clone()); //needs work
        

        
        crate::setup::fire_process_tvshows::process_tvshows(media_lists.1.clone());


        let ab_list = crate::setup::fire_misc::create_art_alb_list(media_lists.0.clone());
        let artist_list = crate::setup::fire_misc::create_artistids(ab_list.0);
        let album_list = crate::setup::fire_misc::create_albumids(ab_list.1);

        let art_serial = serde_json::to_string(&artist_list).unwrap();
        let alb_serial = serde_json::to_string(&album_list);

        println!("artistid_list; {:#?}\n", art_serial);
        println!("albumid_list; {:#?}\n", alb_serial);
        
        println!("music: {}\n", media_lists.0.clone().len());
        println!("videos: {}\n", media_lists.1.clone().len());
        println!("images: {}\n", media_lists.2.clone().len());
        

    
    };

    if scan_home_dir == "no" {

        let famp = env::var("FIRE_ADDITIONAL_MEDIA_PATH")
            .expect("$FIRE_ADDITIONAL_MEDIA_PATH is not set");

        let add_media_list = crate::setup::fire_walk_dirs::walk_additional_dir(famp);
        
        println!("videos: {:?}\n", add_media_list.0.clone().len());
        println!("images: {:?}\n", add_media_list.1.clone().len());
        println!("music: {:?}\n", add_media_list.2.clone().len());
    };
        
    
    
    
    true
}
