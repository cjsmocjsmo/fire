use std::env;
// use std::fs;

fn create_music_thumbnail(x: &String, art: String, alb: String) -> String{
    let fire_music_metadata_path =
        env::var("FIRE_THUMBNAIL").expect("$FIRE_THUMBNAIL is not set");
    // let old_fname = crate::setup::fire_split::split_poster_name(x.clone());
    let new_fname = "/".to_string() + art.as_str() + "_-_" + alb.as_str() + ".jpg";
    let out_fname = fire_music_metadata_path + "/" + &new_fname;

    // println!("this is out_fname:\n\t{}", out_fname);

    let img = image::open(x).expect("ooooh fuck it didnt open");
    let thumbnail = img.resize(200, 200, image::imageops::FilterType::Lanczos3);
    thumbnail.save(out_fname.clone()).expect("Saving image failed");

    out_fname.to_string()
}

pub fn process_music_images(x: String, index: i32) -> String {
    // let mp3_imagesvec = crate::setup::fire_walk_dirs::walk_music_dir_images();

    // let mut image_count = 0;

    // let mut bad_image_vec = vec![];

    // for jpg in mp3_imagesvec {
        

        let id = crate::setup::fire_misc::get_md5(&x);

        let dims = crate::setup::fire_image::get_image_dims(&x);
        if dims != (0, 0) {
            let newdims = crate::setup::fire_image::normalize_music_image(dims);
            let width_r = newdims.0.to_string();
            let height_r = newdims.1.to_string();

            let base_dir = crate::setup::fire_split::split_base_dir(&x);
            let file_name = crate::setup::fire_split::split_filename(&x);
            let extension = crate::setup::fire_split::split_ext(&x);

            let artist_results = crate::setup::fire_split::image_split_artist(&base_dir);
            let album_results = crate::setup::fire_split::image_split_album(&base_dir);

            let fsize_results = crate::setup::fire_misc::get_file_size(&x).to_string();
            let fullpath = &x.to_string();

            let thumb_path = create_music_thumbnail(&x, artist_results.clone(), album_results.clone());

            // let b64image = crate::setup::fire_image::to_base64_str(&thumb_path);

            crate::setup::fire_image::write_image_json_to_file(
                id,
                width_r,
                height_r,
                base_dir,
                file_name,
                extension,
                artist_results,
                album_results,
                fsize_results,
                // b64image,
                fullpath.to_string(),
                index.to_string(),
                thumb_path,
            );
            
        };
        // put it in a db
    

    // let bad_image_count = bad_image_vec.clone().len();

    // if bad_image_count != 0 {
    //     let fire_music_metadata_path =
    //         env::var("fire_MUSIC_METADATA_PATH").expect("$fire_MUSIC_METADATA_PATH is not set");

    //     let a = format!("{}/", fire_music_metadata_path.as_str());
    //     let b = format!("Bad_Music_Images.json");
    //     let outpath = a + &b;
    //     fs::write(outpath, bad_image_vec.join("\n"))
    //         .expect("Failed to write named incorrectly json file");
    // }

    "fuckit".to_string()
}
