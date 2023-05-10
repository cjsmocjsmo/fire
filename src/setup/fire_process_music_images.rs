use std::env;
use json::object;

fn create_music_thumbnail(x: &String, art: String, alb: String) -> String {
    let fire_music_metadata_path = env::var("FIRE_THUMBNAILS").expect("$FIRE_THUMBNAILS is not set");
    let new_fname = "/".to_string() + art.as_str() + "_-_" + alb.as_str() + ".jpg";
    let out_fname = fire_music_metadata_path + &new_fname;

    let img = image::open(x).expect("ooooh fuck it didnt open");
    let thumbnail = img.resize(200, 200, image::imageops::FilterType::Lanczos3);
    thumbnail
        .save(out_fname.clone())
        .expect("Saving image failed");

    out_fname.to_string()
}






// pub fn set_id(x: String) -> String {
//     let foobar = crate::setup::fire_utils::FireUtils {
//         apath: x
//     };
//     let id = crate::setup::fire_utils::FireUtils::get_md5(&foobar);
//     id
//  }

// pub fn get_dims(x: String) -> (u32, u32) {
//     let foobar = crate::setup::fire_utils::FireUtils {
//         apath: x
//     };
//     let dims = crate::setup::fire_image::get_image_dims(&x);
//     dims
//  }


pub fn process_music_images(x: String, index: i32) -> bool {
    // let foo = MusicImageInfo{
    //     imgpath: x.clone()
    // };

    let foo2 = crate::setup::fire_utils::FireUtils {
        apath: x.clone()
    };

    let id = crate::setup::fire_utils::FireUtils::get_md5(&foo2);
    let dims = crate::setup::fire_utils::FireUtils::get_dims(&foo2);

    if dims != (0, 0) {
        let newdims = crate::setup::fire_image::normalize_music_image(dims);
        let width_r = newdims.0.to_string();
        let height_r = newdims.1.to_string();

        let base_dir = crate::setup::fire_utils::FireUtils::split_base_dir(&foo2);

        let file_name = crate::setup::fire_utils::FireUtils::split_filename(&foo2);

        let ext = crate::setup::fire_utils::FireUtils::split_ext(&foo2);
        // let ext = MusicImageInfo::get_ext(&foo);

        let artist_results = crate::setup::fire_utils::FireUtils::image_split_artist(&foo2);
        let album_results = crate::setup::fire_utils::FireUtils::image_split_album(&foo2);
        println!("\n\n art && album: {} {}\n\n", artist_results.clone(), album_results.clone());

        let fsize_results = crate::setup::fire_utils::FireUtils::get_file_size(&foo2).to_string();
        let full_path = &x.to_string();

        let thumb_path = create_music_thumbnail(&x, artist_results.clone(), album_results.clone());

        println!("\n\nthumbpath: {}\n\n", thumb_path);

        // let b64image = crate::setup::fire_image::to_base64_str(&thumb_path);

        let music_image_info = object! {
            id: id,
            width: width_r,
            height: height_r,
            basedir: base_dir,
            filename: file_name,
            extension: ext,
            artist: artist_results,
            album: album_results,
            filesize: fsize_results,
            fullpath: full_path.to_string(),
            thumbpath: thumb_path,
            index: index.to_string(),
        };

        let mii = json::stringify(music_image_info.dump());
        println!("\n{:?}", mii);
        // return music_image_info;
    };

    // music_image_info
    true
}
