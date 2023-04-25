// use std::fs;
use std::env;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub mod fire_env_vars;
pub mod fire_image;
pub mod fire_misc;
pub mod fire_mp3_info;
pub mod fire_process_movie_images;
pub mod fire_process_music;
pub mod fire_process_music_images;
pub mod fire_split;
pub mod fire_walk_dirs;

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
        println!("{:?}", ifo);
    }
}

fn run_img_threads(alist: Vec<String>) {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    let mut img_index = 0;
    for i in alist {
        img_index = img_index + 1;
        if i.contains("Posters2") {
            let tx = tx.clone();
            pool.execute(move || {
                let img_info =
                    fire_process_movie_images::process_movie_posters(i.clone(), img_index);
                tx.send(img_info).expect("Could not send data");
                println!("{}", i.clone());
            });
        } else if i.contains("Music") {
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
        println!("{}", ifo);
    }
}



pub fn run_setup() -> bool {
    let paramaters = fire_env_vars::read_config();

    crate::setup::fire_env_vars::set_all_env_vars(paramaters);

    let media_lists = fire_walk_dirs::scan_all_sources();

    run_music_threads(media_lists.0);

    run_img_threads(media_lists.2);


    let video_list = media_lists.1;
    for v in video_list {
        println!("{}\n", v);
    };

    true
}
