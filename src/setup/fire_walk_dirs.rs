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
    let mut mov_posters_vec = Vec::new();
    let mut tv_posters_vec = Vec::new();
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
                if fname.contains("MovPosters") {
                    mov_posters_vec.push(fname);
                } else if fname.contains("TVPosters") {
                    tv_posters_vec.push(fname);
                } else if fname.contains("Music") {
                    musicimgvec.push(fname);
                } else {
                    continue;
                };
            } else if fname.ends_with(".jpeg") {
                if fname.contains("MovPosters") {
                    mov_posters_vec.push(fname);
                } else if fname.contains("TVPosters") {
                    tv_posters_vec.push(fname);
                } else if fname.contains("Music") {
                    musicimgvec.push(fname);
                } else {
                    continue;
                };
            } else if fname.ends_with(".png") {
                if fname.contains("MovPosters") {
                    mov_posters_vec.push(fname);
                } else if fname.contains("TVPosters") {
                    tv_posters_vec.push(fname);
                } else if fname.contains("Music") {
                    musicimgvec.push(fname);
                } else {
                    continue;
                };
            } else if fname.ends_with(".webp") {
                if fname.contains("MovPosters") {
                    mov_posters_vec.push(fname);
                } else if fname.contains("TVPosters") {
                    tv_posters_vec.push(fname);
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
    media_images.append(&mut mov_posters_vec);
    media_images.append(&mut tv_posters_vec);
    media_images.append(&mut musicimgvec);

    (video_list.clone(), media_images.clone(), musicvec.clone())
}

fn scan_home_dir() -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    let homedir = home_dir();
    let music_dir = homedir.clone() + "/Music";
    let music_list = walk_music_dir_music(music_dir.clone());

    let video_dir = homedir.clone() + "/Videos";
    let video_list = walk_video_dir(video_dir.clone());

    let vid_posters_path = video_dir.clone() + "/MovPosters";
    let vid_posters = fire_walk_dirs::walk_posters2_dir(vid_posters_path.clone());

    let music_images = fire_walk_dirs::walk_music_dir_images(music_dir.clone());

    (music_list, video_list, vid_posters, music_images)
}

fn scan_usb1() -> (Vec<String>, Vec<String>, Vec<String>) {
    let usb1 = env::var("FIRE_USB1").expect("$FIRE_USB1 is not set");
    let add_media = fire_walk_dirs::walk_additional_dir(usb1);
    let add_video_list = add_media.0;
    let add_media_img_list = add_media.1;
    let add_music_list = add_media.2;

    (add_music_list, add_video_list, add_media_img_list)
}

fn scan_usb2() -> (Vec<String>, Vec<String>, Vec<String>) {
    let usb2 = env::var("FIRE_USB2").expect("$FIRE_USB2 is not set");
    let add_media = fire_walk_dirs::walk_additional_dir(usb2);
    let add_video_list = add_media.0;
    let add_media_img_list = add_media.1;
    let add_music_list = add_media.2;

    (add_music_list, add_video_list, add_media_img_list)
}

fn scan_usb3() -> (Vec<String>, Vec<String>, Vec<String>) {
    let usb3 = env::var("FIRE_USB3").expect("$FIRE_USB3 is not set");
    let add_media = fire_walk_dirs::walk_additional_dir(usb3);
    let add_video_list = add_media.0;
    let add_media_img_list = add_media.1;
    let add_music_list = add_media.2;

    (add_music_list, add_video_list, add_media_img_list)
}

fn scan_usb4() -> (Vec<String>, Vec<String>, Vec<String>) {
    let usb4 = env::var("FIRE_USB4").expect("$FIRE_USB4 is not set");
    let add_media = fire_walk_dirs::walk_additional_dir(usb4);
    let add_video_list = add_media.0;
    let add_media_img_list = add_media.1;
    let add_music_list = add_media.2;

    (add_music_list, add_video_list, add_media_img_list)
}

pub fn scan_all_sources() -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut master_music_list = Vec::new();
    let mut master_video_list = Vec::new();
    let mut master_img_list = Vec::new();

    let scan_hd = env::var("FIRE_SCAN_HOME").expect("not set");
    if scan_hd != "yes".to_string() {
        let hd = scan_home_dir();
        let mut music_list = hd.0;
        let mut video_list = hd.1;
        let mut vid_posters = hd.2;
        let mut music_images = hd.3;
        master_music_list.append(&mut music_list);
        master_video_list.append(&mut video_list);
        master_img_list.append(&mut vid_posters);
        master_img_list.append(&mut music_images);
    }
    
    let usb1_var = env::var("FIRE_USB1").expect("Not set");
    if usb1_var != "None".to_string() {
        let usb1 = scan_usb1();
        let mut usb1_music_list = usb1.0;
        let mut usb1_video_list = usb1.1;
        let mut usb1_media_images = usb1.2;
        master_music_list.append(&mut usb1_music_list);
        master_video_list.append(&mut usb1_video_list);
        master_img_list.append(&mut usb1_media_images);
    }
    
    let usb2_var = env::var("FIRE_USB2").expect("not set");
    if usb2_var != "None".to_string() {
        let usb2 = scan_usb2();
        let mut usb2_music_list = usb2.0;
        let mut usb2_video_list = usb2.1;
        let mut usb2_media_iamges = usb2.2;
        master_music_list.append(&mut usb2_music_list);
        master_video_list.append(&mut usb2_video_list);
        master_img_list.append(&mut usb2_media_iamges);
    }
    
    let usb3_var = env::var("FIRE_USB3").expect("not set");
    if usb3_var != "None".to_string() {
        let usb3 = scan_usb3();
        let mut usb3_music_list = usb3.0;
        let mut usb3_video_list = usb3.1;
        let mut usb3_media_iamges = usb3.2;
        master_music_list.append(&mut usb3_music_list);
        master_video_list.append(&mut usb3_video_list);
        master_img_list.append(&mut usb3_media_iamges);
    }

    let usb4_var = env::var("FIRE_USB4").expect("not set");
    if usb4_var != "None".to_string() {
        let usb4 = scan_usb4();
        let mut usb4_music_list = usb4.0;
        let mut usb4_video_list = usb4.1;
        let mut usb4_media_iamges = usb4.2;
        master_music_list.append(&mut usb4_music_list);
        master_video_list.append(&mut usb4_video_list);
        master_img_list.append(&mut usb4_media_iamges);

    }

    println!("this is media_list: {}", master_music_list.clone().len());
    println!("this is video_list: {}", master_video_list.clone().len());
    println!("this is media_images: {}", master_img_list.clone().len());

    (master_music_list, master_video_list, master_img_list)

}
