use std::env;


fn create_music_thumbnail(x: &String, art: String, alb: String) -> String {
    let fire_music_metadata_path = env::var("FIRE_THUMBNAIL").expect("$FIRE_THUMBNAIL is not set");
    let new_fname = "/".to_string() + art.as_str() + "_-_" + alb.as_str() + ".jpg";
    let out_fname = fire_music_metadata_path + "/" + &new_fname;

    let img = image::open(x).expect("ooooh fuck it didnt open");
    let thumbnail = img.resize(200, 200, image::imageops::FilterType::Lanczos3);
    thumbnail
        .save(out_fname.clone())
        .expect("Saving image failed");

    out_fname.to_string()
}







#[derive(Debug)]
pub struct MusicImageInfo {
    imgpath: String
    
}

#[derive(Debug)]
pub struct MII {
    id: String,
    width: String,
    height: String,
    basedir: String,
    filename: String,
    extension: String,
    artist: String,
    album: String,
    filesize: String,
    fullpath: String,
    thumbpath: String,
    index: String,
}

impl MusicImageInfo {
    fn set_id(&self) -> String {
        let id = crate::setup::fire_misc::get_md5(&self.imgpath);
        id
     }
     fn get_dims(&self) -> (u32, u32) {
        let dims = crate::setup::fire_image::get_image_dims(&self.imgpath);
        dims
     }
     fn get_base_dir(&self) -> String {
        let bdir = crate::setup::fire_split::split_base_dir(&self.imgpath);
        bdir
     }
     fn get_file_name(&self) -> String {
        let fname = crate::setup::fire_split::split_filename(&self.imgpath);
        fname
     }
     fn get_ext(&self) -> String {
        let ext = crate::setup::fire_split::split_ext(&self.imgpath);
        ext
     }
     
}

pub fn process_music_images(x: String, index: i32) -> bool {
    let foo = MusicImageInfo{
        imgpath: x.clone()
    };

    let id = MusicImageInfo::set_id(&foo);
    let dims = MusicImageInfo::get_dims(&foo);

    if dims != (0, 0) {
        let newdims = crate::setup::fire_image::normalize_music_image(dims);
        let width_r = newdims.0.to_string();
        let height_r = newdims.1.to_string();

        let base_dir = MusicImageInfo::get_base_dir(&foo);

        let file_name = MusicImageInfo::get_file_name(&foo);
        let ext = MusicImageInfo::get_ext(&foo);

        let artist_results = crate::setup::fire_split::image_split_artist(&base_dir);
        let album_results = crate::setup::fire_split::image_split_album(&base_dir);

        let fsize_results = crate::setup::fire_misc::get_file_size(&x).to_string();
        let full_path = &x.to_string();

        let thumb_path = create_music_thumbnail(&x, artist_results.clone(), album_results.clone());

        // let b64image = crate::setup::fire_image::to_base64_str(&thumb_path);

        let music_image_info = MII {
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
        println!("\n{:?}", music_image_info);
        // return music_image_info;
    };

    // music_image_info
    true
}
