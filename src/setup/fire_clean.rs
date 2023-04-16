use glob::glob;
use std::env;
use std::fs;

// fn clean_nfos_dir() -> u32 {
//     let movie_meta_dir_path = env::var("FIRE_NFOS").unwrap();
//     let glob_str = movie_meta_dir_path + "/*.nfo";
//     let mut count = 0;
//     for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
//         let rm_path = e.unwrap();
//         count = count + 1;
//         fs::remove_file(rm_path).expect("File delete failed");
//     }

//     count
// }

// fn clean_thumbnail_dir() -> u32 {
//     let music_meta_dir_path = env::var("FIRE_THUMBNAIL").unwrap();
//     let glob_str = music_meta_dir_path + "/*.jpg";
//     let mut count = 0;
//     for e in glob(glob_str.as_str()).expect("Failed to read glob pattern") {
//         count = count + 1;
//         let rm_path = e.unwrap();
//         fs::remove_file(rm_path).expect("File delete failed");
//     }
//     count
// }

// pub fn clean_meta() -> u32 {
//     let nfos = clean_nfos_dir();
//     let thumbs = clean_thumbnail_dir();
//     let tot = nfos + thumbs;

//     tot
// }
