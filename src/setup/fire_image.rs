// use base64::{alphabet, engine, Engine as _};
use image::{self};
use json::object;
use std::env;

pub fn get_image_dims(x: &String) -> (u32, u32) {
    let dims_rs = image::image_dimensions(&x);
    let dims = match dims_rs {
        Ok(d) => d,
        Err(_) => (0, 0)
    };

    dims
}

pub fn normalize_music_image(dims: (u32, u32)) -> (u32, u32) {
    let largest: u32;

    if dims.0 == dims.1 {
        largest = dims.0;
    } else if dims.0 > dims.1 {
        largest = dims.0;
    } else {
        largest = dims.1;
    }

    let resizetup: (u32, u32);
    if largest < 100 {
        resizetup = (100, 100);
    } else if largest < 200 {
        resizetup = (200, 200);
    } else if largest < 300 {
        resizetup = (300, 300);
    } else {
        resizetup = (300, 300);
    }

    resizetup
}

// pub fn to_base64_str(x: &String) -> String {
//     let img_result = image::open(&x);
//     let img = match img_result {
//         Ok(img) => img,
//         Err(error) => panic!("problem opening file {:?}", error),
//     };
//     // let thumb = img.thumbnail(w, h);
//     let thumb_bytes = img.into_bytes();

//     let alphabet =
//         alphabet::Alphabet::new("+/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
//             .unwrap();

//     // a very weird config that encodes with padding but requires no padding when decoding...?
//     let crazy_config = engine::GeneralPurposeConfig::new()
//         .with_decode_allow_trailing_bits(true)
//         .with_encode_padding(true)
//         .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

//     let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);

//     let encoded = crazy_engine.encode(thumb_bytes);

//     encoded
// }

pub fn write_image_json_to_file(
    id: String,
    width: String,
    height: String,
    base_dir: String,
    file_name: String,
    extension: String,
    artist_results: String,
    album_results: String,
    fsize_results: String,
    // b64image: String,
    fullpath: String,
    imagecount: String,
    thumbnail_path: String,
) {
    let imginfo = object! {
        imageid: id,
        filename_artist: artist_results,
        filename_album: album_results,
        basedir: base_dir,
        filename: file_name,
        ext: extension,
        width: width,
        height: height,
        idx: &*imagecount,
        fsize: fsize_results,
        fullpath: fullpath,
        // b64img: b64image,
        thumbpath: thumbnail_path
    };

    let ifo = json::stringify(imginfo.dump());

    let mtv_music_metadata_path =
        env::var("MTV_MUSIC_METADATA_PATH").expect("$MTV_MUSIC_METADATA_PATH is not set");

    let a = format!("{}/", mtv_music_metadata_path.as_str());
    let b = format!("Music_Image_Meta_{}.json", &imagecount);
    let outpath = a + &b;

    // println!("\n\n\n ifo {:#?}", ifo);
    std::fs::write(outpath, ifo).unwrap();
}
