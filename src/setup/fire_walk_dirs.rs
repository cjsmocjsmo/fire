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

pub fn walk_additional_dir(apath: String) -> (Vec<String>,Vec<String>,Vec<String>) {
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
                } else if fname.contains("Posters") {
                    musicimgvec.push(fname);
                } else if fname.contains("Music") {
                    musicimgvec.push(fname);
                } else {
                    continue;
                };
            } else if fname.ends_with(".jpeg") {
                if fname.contains("Posters2") {
                    posters2vec.push(fname);
                } else if fname.contains("Posters") {
                    musicimgvec.push(fname);
                } else if fname.contains("Music") {
                    musicimgvec.push(fname);
                } else {
                    continue;
                };
            } else if fname.ends_with(".png") {
                if fname.contains("Posters2") {
                    posters2vec.push(fname);
                } else if fname.contains("Posters") {
                    musicimgvec.push(fname);
                } else if fname.contains("Music") {
                    musicimgvec.push(fname);
                } else {
                    continue;
                };
            } else if fname.ends_with(".webp") {
                if fname.contains("Posters2") {
                    posters2vec.push(fname);
                } else if fname.contains("Posters") {
                    musicimgvec.push(fname);
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
    };

    let mut video_list = Vec::new();
    video_list.append(&mut moviesvec);
    video_list.append(&mut tvshowsvec);

    let mut media_images = Vec::new();
    media_images.append(&mut posters2vec);
    media_images.append(&mut musicimgvec);

    (video_list.clone(), media_images.clone(), musicvec.clone())
}

pub fn scan_all_sources() -> (Vec<String>, Vec<String>, Vec<String>) {
    let homedir = home_dir();
    
    let music_dir = homedir.clone() + "/Music";
    let mut music_list = walk_music_dir_music(music_dir.clone());

    let video_dir = homedir.clone() + "/Videos";
    let mut video_list = walk_video_dir(video_dir.clone());

    let vid_posters_path = video_dir.clone() + "/MovPosters";
    let mut vid_posters = fire_walk_dirs::walk_posters2_dir(vid_posters_path.clone());

    let mut music_images = fire_walk_dirs::walk_music_dir_images(music_dir.clone());
    
    println!("this is posters count: {}", vid_posters.len());
    println!("this is music images: {}", music_images.len());

    let mut media_images = Vec::new();
    
    media_images.append(&mut vid_posters);
    media_images.append(&mut music_images);

    let usb1 = env::var("FIRE_USB1").expect("$FIRE_USB1 is not set");
    let add_media = fire_walk_dirs::walk_additional_dir(usb1);
    let mut add_video_list = add_media.0;
    let mut add_media_img_list = add_media.1;
    let mut add_music_list = add_media.2;

    video_list.append(&mut add_video_list);
    music_list.append(&mut add_music_list);
    media_images.append(&mut add_media_img_list);
    
    let usb2 = env::var("FIRE_USB2").expect("$FIRE_USB2 is not set");
    let add_media2 = fire_walk_dirs::walk_additional_dir(usb2);
    let mut add_video_list2 = add_media2.0;
    let mut add_media_img_list2 = add_media2.1;
    let mut add_music_list2 = add_media2.2;

    video_list.append(&mut add_video_list2);
    music_list.append(&mut add_music_list2);
    media_images.append(&mut add_media_img_list2);


    let usb3 = env::var("FIRE_USB3").expect("$FIRE_USB3 is not set");
    let add_media3 = fire_walk_dirs::walk_additional_dir(usb3);
    let mut add_video_list3 = add_media3.0;
    let mut add_media_img_list3 = add_media3.1;
    let mut add_music_list3 = add_media3.2;

    video_list.append(&mut add_video_list3);
    music_list.append(&mut add_music_list3);
    media_images.append(&mut add_media_img_list3);


    let usb4 = env::var("FIRE_USB4").expect("$FIRE_USB4 is not set");
    let add_media4 = fire_walk_dirs::walk_additional_dir(usb4);
    let mut add_video_list4 = add_media4.0;
    let mut add_media_img_list4 = add_media4.1;
    let mut add_music_list4 = add_media4.2;

    video_list.append(&mut add_video_list4);
    music_list.append(&mut add_music_list4);
    media_images.append(&mut add_media_img_list4);

    (music_list, video_list, media_images)

}
