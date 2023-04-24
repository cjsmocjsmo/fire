// use image::{self};


use json::object;
use std::env;
// use std::fs;

fn create_movie_thumbnail(x: String) -> String {
    let fire_movie_metadata_path =
        env::var("FIRE_THUMBNAIL").expect("$FIRE_THUMBNAIL is not set");
    let old_fname = crate::setup::fire_split::split_poster_name(x.clone());
    let out_fname = fire_movie_metadata_path + "/" + &old_fname;

    let img = image::open(x).expect(&x);
    let thumbnail = img.resize(230, 345, image::imageops::FilterType::Lanczos3);
    thumbnail
        .save(out_fname.clone())
        .expect("Saving image failed");

    out_fname
}

pub fn process_movie_posters(x: String, index: i32) -> (String, String) {
    // let movie_posters_vec = crate::fire_walk_dirs::walk_posters2_dir();
    // let mut index = 0;
    // let mut bad_image_vec = vec![];

    // for x in movie_posters_vec {
    // index = index + 1;

    let dims = crate::setup::fire_image::get_image_dims(&x);
    println!("{:?}", dims);
    println!("{}", index);

    let img_size = crate::setup::fire_misc::get_file_size(&x);
    let name = crate::setup::fire_split::split_poster_name(x.clone());
    let thumb_path = create_movie_thumbnail(x.clone());

    let mov_img_obj = object! {
        path: &*x,
        size: img_size.to_string(),
        name: name,
        thumbpath: thumb_path,
    };

    let mov_img_info = json::stringify(mov_img_obj.dump());

    println!("{}", mov_img_info);

    // let fire_movie_metadata_path =
        // env::var("fire_MOVIES_METADATA_PATH").expect("$fire_MOVIES_METADATA_PATH is not set");

    // let a = format!("{}/", fire_movie_metadata_path.as_str());
    // let b = format!("Movie_Image_{}_Info.json", index.to_string());
    // let outpath = a + &b;
    // fs::write(outpath, mov_img_info).expect("Failed to write named incorrectly json file");
   
    // else {
    //     bad_image_vec.push(x.clone());

    //     println!("this is a bad image:\n\t {}", x.clone());
    // }
    
    // let bad_image_count = bad_image_vec.clone().len();

    // if bad_image_count != 0 {
    //     let fire_movie_metadata_path =
    //         env::var("fire_MOVIES_METADATA_PATH").expect("$fire_MOVIES_METADATA_PATH is not set");

    //     let a = format!("{}/", fire_movie_metadata_path.as_str());
    //     let b = format!("Bad_Movies_Images.json");
    //     let outpath = a + &b;
    //     fs::write(outpath, bad_image_vec.join("\n"))
    //         .expect("Failed to write named incorrectly json file");
    // }

    // (bad_image_count.to_string(), index.to_string())
    (String::from("fuckit"), String::from("fuckit2"))
}
