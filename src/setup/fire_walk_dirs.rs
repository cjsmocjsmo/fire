use std::env;
use walkdir::WalkDir;

use crate::setup::fire_walk_dirs;
// use simple_home_dir::*;

fn home_dir() -> String {
    let hd = simple_home_dir::home_dir().unwrap().to_string_lossy().to_string();
    return hd
}

fn walk_video_dir(apath: String) -> Vec<String> {
    let mut vidvec = Vec::new();
    // let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.ends_with("mp4") {
                vidvec.push(fname.clone());
            } else if fname.ends_with("mkv") {
                vidvec.push(fname.clone());
            } else {
                continue;
            }
        }
    }

    vidvec
}


fn walk_music_dir_music(apath: String) -> Vec<String> {
    let mut vidvec = Vec::new();
    // let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.ends_with(".mp3") {
                vidvec.push(fname.clone());
            } else {
                continue;
            }
        }
    }

    vidvec
}

fn walk_music_dir_images(apath: String) -> Vec<String> {
    let mut musicimagevec = Vec::new();
    // let mtv_music_path = env::var("MTV_MUSIC_PATH").expect("$MTV_MUSIC_PATH is not set");
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.ends_with(".jpg") {
                musicimagevec.push(fname);
            } else if fname.ends_with(".jpeg") {
                musicimagevec.push(fname);
            } else if fname.ends_with(".png") {
                musicimagevec.push(fname);
            } else if fname.ends_with(".webp") {
                musicimagevec.push(fname);
            } else if fname.ends_with(".avif") {
                musicimagevec.push(fname);
            } else {
                continue;
            }
        }
    }

    musicimagevec
}

fn walk_posters2_dir(apath: String) -> Vec<String> {
    let mut moviesthumbvec = Vec::new();
    // let mtv_movies_thumb_path =
    //     env::var("MTV_MOVIES_POSTERS_PATH").expect("$MTV_MOVIES_POSTERS_PATH is not set");
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            if fname.ends_with(".jpg") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".jpeg") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".png") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".webp") {
                moviesthumbvec.push(fname);
            } else if fname.ends_with(".avif") {
                moviesthumbvec.push(fname);
            } else {
                continue;
            }
        }
    }

    moviesthumbvec
}

pub fn scan_all_sources() {
    let homedir = home_dir();
    // println!("this is home dir {}", homedir.clone());
    
    let music_dir = homedir.clone() + "/Music";
    let mut music_list = walk_music_dir_music(music_dir.clone());

    // walk FIRE_ADD_MUSIC_PATH
    let add_music = env::var("FIRE_ADD_MUSIC_PATH").expect("$FIRE_ADD_MUSIC_PATH is not set");
    let mut mlist2 = Vec::new();
    if add_music != String::from("NONE") {
        mlist2 = walk_music_dir_music(add_music);
    };

    music_list.append(&mut mlist2);

    println!("{:?}", music_list);

    






    let video_dir = homedir.clone() + "/Videos";
    let mut video_list = walk_video_dir(video_dir.clone());
    
    // walk FIRE_ADD_VIDEO_PATH
    let add_vids = env::var("FIRE_ADD_VIDEO_PATH").expect("$FIRE_ADD_VIDEO_PATH is not set");
    let mut vlist2 = Vec::new();
    if add_vids != String::from("NONE") {
        vlist2 = walk_music_dir_music(add_vids);
    };

    video_list.append(&mut vlist2);
    
    println!("{:?}", video_list);

    // scan posters2 
    // scan mp3 folders for images

    let mut media_images = Vec::new();

    let vid_posters_path = video_dir.clone() + "Posters2";
    let mut vid_posters = fire_walk_dirs::walk_posters2_dir(vid_posters_path);

    let mut music_images = fire_walk_dirs::walk_music_dir_images(music_dir.clone());

    media_images.append(&mut vid_posters);
    media_images.append(&mut music_images);

    println!("{:?}", media_images);

}


// fn walk_movies_dir() -> Vec<String> {
//     let mut moviesvec = Vec::new();
//     let mtv_movies_path = env::var("MTV_MOVIES_PATH").expect("$MTV_MOVIES_PATH is not set");
//     for e in WalkDir::new(mtv_movies_path.clone())
//         .follow_links(true)
//         .into_iter()
//         .filter_map(|e| e.ok())
//     {
//         if e.metadata().unwrap().is_file() {
//             let fname = e.path().to_string_lossy().to_string();
//             // println!("this is movie file:\n\t{}\n", fname.clone());
//             if fname.ends_with(".mp4") {
//                 moviesvec.push(fname.clone());
//             } else if fname.ends_with(".mkv") {
//                 moviesvec.push(fname.clone());
//             } else {
//                 continue;
//             }
//         }
//     }

//     moviesvec
// }



// fn walk_tvshows_dir() -> Vec<String> {
//     let mut tvshowsvec = Vec::new();
//     let mtv_tvshows_path = env::var("MTV_TVSHOWS_PATH").expect("$MTV_TVSHOWS_PATH is not set");
//     for e in WalkDir::new(mtv_tvshows_path.clone())
//         .follow_links(true)
//         .into_iter()
//         .filter_map(|e| e.ok())
//     {
//         if e.metadata().unwrap().is_file() {
//             let fname = e.path().to_string_lossy().to_string();

//             if fname.ends_with(".mp4") {
//                 tvshowsvec.push(fname);
//             } else if fname.ends_with(".mkv") {
//                 tvshowsvec.push(fname);
//             } else {
//                 continue;
//             }
//         }
//     }

//     tvshowsvec
// }