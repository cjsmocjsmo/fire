use std::env;
use std::fs;

fn create_music_thumbnail(x: &String, art: String, alb: String) -> String{
    let mtv_music_metadata_path =
        env::var("MTV_MUSIC_THUMBNAIL_PATH").expect("$MTV_MUSIC_THUMBNAIL_PATH is not set");
    // let old_fname = crate::mtv_split::split_poster_name(x.clone());
    let new_fname = "/".to_string() + art.as_str() + "_-_" + alb.as_str() + ".jpg";
    let out_fname = mtv_music_metadata_path + "/" + &new_fname;

    // println!("this is out_fname:\n\t{}", out_fname);

    let img = image::open(x).expect("ooooh fuck it didnt open");
    let thumbnail = img.resize(200, 200, image::imageops::FilterType::Lanczos3);
    thumbnail.save(out_fname.clone()).expect("Saving image failed");

    out_fname.to_string()
}

pub fn process_music_images() -> (String, String) {
    let mp3_imagesvec = crate::mtv_walk_dirs::walk_music_dir_images();

    let mut image_count = 0;

    let mut bad_image_vec = vec![];

    for jpg in mp3_imagesvec {
        image_count = image_count + 1;

        let id = crate::mtv_misc::get_md5(&jpg);

        let dims = crate::mtv_image::get_image_dims(&jpg);
        if dims != (0, 0) {
            let newdims = crate::mtv_image::normalize_music_image(dims);
            let width_r = newdims.0.to_string();
            let height_r = newdims.1.to_string();

            let base_dir = crate::mtv_split::split_base_dir(&jpg);
            let file_name = crate::mtv_split::split_filename(&jpg);
            let extension = crate::mtv_split::split_ext(&jpg);

            let artist_results = crate::mtv_split::image_split_artist(&base_dir);
            let album_results = crate::mtv_split::image_split_album(&base_dir);

            let fsize_results = crate::mtv_misc::get_file_size(&jpg).to_string();
            let fullpath = &jpg.to_string();

            let thumb_path = create_music_thumbnail(&jpg, artist_results.clone(), album_results.clone());

            // let b64image = crate::mtv_image::to_base64_str(&thumb_path);

            crate::mtv_image::write_image_json_to_file(
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
                image_count.to_string(),
                thumb_path,
            );
            
        } else {
            bad_image_vec.push(jpg.clone());

            println!("this is a bad image:\n\t {}", jpg.clone());
        };
        // put it in a db
    }

    let bad_image_count = bad_image_vec.clone().len();

    if bad_image_count != 0 {
        let mtv_music_metadata_path =
            env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

        let a = format!("{}/", mtv_music_metadata_path.as_str());
        let b = format!("Bad_Music_Images.json");
        let outpath = a + &b;
        fs::write(outpath, bad_image_vec.join("\n"))
            .expect("Failed to write named incorrectly json file");
    }

    (image_count.to_string(), bad_image_count.to_string())
}
