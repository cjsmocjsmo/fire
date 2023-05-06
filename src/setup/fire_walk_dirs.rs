// use std::env;
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
    let mut mp3vec = Vec::new();
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.ends_with(".mp3") {
                mp3vec.push(fname.clone());
            } else {
                continue;
            }
        }
    }

    mp3vec
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
            } else {
                continue;
            }
        }
    }

    moviesthumbvec
}

pub fn walk_additional_dir(apath: String) -> (Vec<String>, Vec<String>, Vec<String>,Vec<String>,Vec<String>) {
    let mut moviesvec = Vec::new();
    let mut tvshowsvec = Vec::new();
    let mut posters2vec = Vec::new();
    let mut musicvec = Vec::new();
    let mut musicimgvec = Vec::new();

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            if fname.ends_with(".jpg") {
                if fname.contains("Posters2") {
                    posters2vec.push(fname);
                } else if fname.contains("Music") {
                    musicimgvec.push(fname);
                } else {
                    continue;
                };
            } else if fname.ends_with(".jpeg") {
                if fname.contains("Posters2") {
                    posters2vec.push(fname);
                } else if fname.contains("Music") {
                    musicimgvec.push(fname);
                } else {
                    continue;
                };
            } else if fname.ends_with(".png") {
                if fname.contains("Posters2") {
                    posters2vec.push(fname);
                } else if fname.contains("Music") {
                    musicimgvec.push(fname);
                } else {
                    continue;
                };
            } else if fname.ends_with(".webp") {
                if fname.contains("Posters2") {
                    posters2vec.push(fname);
                } else if fname.contains("Music") {
                    musicimgvec.push(fname);
                } else {
                    continue;
                };
            } else if fname.ends_with(".mkv") {
                if fname.contains("Movies") {
                    moviesvec.push(fname);
                } else if fname.contains("TVShows") {
                    tvshowsvec.push(fname);
                };
            } else if fname.ends_with(".mp4") {
                if fname.contains("Movies") {
                    moviesvec.push(fname);
                } else if fname.contains("TVShows") {
                    tvshowsvec.push(fname);
                };
            } else if fname.ends_with(".mp3") {
                musicvec.push(fname);
            } else {
                continue;
            }
        }
    }
    println!("{}", moviesvec.clone().len());
    println!("{}", tvshowsvec.clone().len());
    println!("{}", posters2vec.clone().len());
    println!("{}", musicvec.clone().len());
    println!("{}", musicimgvec.clone().len());

    (moviesvec.clone(), tvshowsvec.clone(), posters2vec.clone(), musicvec.clone(), musicimgvec.clone())
}

pub fn scan_all_sources() -> (Vec<String>, Vec<String>, Vec<String>) {
    let homedir = home_dir();
    
    let music_dir = homedir.clone() + "/Music";
    let music_list = walk_music_dir_music(music_dir.clone());

    let video_dir = homedir.clone() + "/Videos";
    let video_list = walk_video_dir(video_dir.clone());

    let mut media_images = Vec::new();

    let vid_posters_path = video_dir.clone() + "/Posters2";
    let mut vid_posters = fire_walk_dirs::walk_posters2_dir(vid_posters_path.clone());

    let mut music_images = fire_walk_dirs::walk_music_dir_images(music_dir.clone());
    
    println!("this is posters count: {}", vid_posters.len());
    println!("this is music images: {}", music_images.len());
    
    media_images.append(&mut vid_posters);
    media_images.append(&mut music_images);

    (music_list, video_list, media_images)

}
