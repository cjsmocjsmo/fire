use std::env;
use walkdir::WalkDir;
use crate::setup::fire_walk_dirs;

fn home_dir() -> String {
    let hd = simple_home_dir::home_dir().unwrap().to_string_lossy().to_string();
    return hd
}

fn walk_video_dir(apath: String) -> Vec<String> {
    let mut vidvec = Vec::new();
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

pub fn scan_all_sources() -> (Vec<String>, Vec<String>, Vec<String>) {
    let homedir = home_dir();
    
    let music_dir = homedir.clone() + "/Music";
    let mut music_list = walk_music_dir_music(music_dir.clone());

    let add_music = env::var("FIRE_ADD_MUSIC_PATH").expect("$FIRE_ADD_MUSIC_PATH is not set");
    let mut mlist2 = Vec::new();
    if add_music != String::from("NONE") {
        mlist2 = walk_music_dir_music(add_music.clone());
    };

    music_list.append(&mut mlist2);

    let video_dir = homedir.clone() + "/Videos";
    let mut video_list = walk_video_dir(video_dir.clone());

    let add_vids = env::var("FIRE_ADD_VIDEO_PATH").expect("$FIRE_ADD_VIDEO_PATH is not set");
    let mut vlist2 = Vec::new();
    if add_vids.clone() != String::from("NONE") {
        vlist2 = walk_music_dir_music(add_vids.clone());
    };

    video_list.append(&mut vlist2);

    let mut media_images = Vec::new();

    let vid_posters_path = video_dir.clone() + "/Posters2";
    println!("{}", vid_posters_path);
    let mut vid_posters = Vec::new();
    if add_vids.clone() != String::from("NONE") {
        vid_posters = fire_walk_dirs::walk_posters2_dir(vid_posters_path);
        println!("{:?}", vid_posters);
    };

    let mut music_images = Vec::new();
    if add_music.clone() != String::from("NONE") {
        music_images = fire_walk_dirs::walk_music_dir_images(music_dir.clone());
    };
    
    media_images.append(&mut vid_posters);
    media_images.append(&mut music_images);

    (music_list, video_list, media_images)

}
