use std::fs;
use std::env;
use std::clone::Clone;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MovPosterInfo {
    id: String,
    path: String,
    dims: String,
    size: String,
    name: String,
    thumbpath: String
}

fn create_movie_thumbnail(x: String) -> String {
    let foobar = crate::setup::fire_utils::FireUtils {
        apath: x.clone()
    };
    let fire_movie_metadata_path =
        env::var("FIRE_THUMBNAILS").expect("$FIRE_THUMBNAILS is not set");
    let old_fname = crate::setup::fire_utils::FireUtils::split_poster_name(&foobar);
    let out_fname = fire_movie_metadata_path + "/" + &old_fname;
    let img = image::open(&x).expect(&x);
    let thumbnail = img.resize(230, 345, image::imageops::FilterType::Lanczos3);
    thumbnail
        .save(out_fname.clone())
        .expect("Saving image failed");

    out_fname
}

pub fn process_movie_posters(x: String, index: i32) -> bool {
    // let movie_posters_vec = crate::fire_walk_dirs::walk_posters2_dir();
    // let mut index = 0;
    // let mut bad_image_vec = vec![];

    // for x in movie_posters_vec {
    // index = index + 1;

    let dims = crate::setup::fire_image::get_image_dims(&x);
    let dims_foo = format!("{:?}", dims);
    println!("{}", index);

    let foobar2 = crate::setup::fire_utils::FireUtils {
        apath: x.clone()
    };

    let img_size = crate::setup::fire_utils::FireUtils::get_file_size(&foobar2);
    let name = crate::setup::fire_utils::FireUtils::split_poster_name(&foobar2);
    let thumb_path = create_movie_thumbnail(x.clone());

    let mov_img_info = MovPosterInfo {
        id: index.to_string(),
        path: x,
        dims: dims_foo,
        size: img_size.to_string(),
        name: name,
        thumbpath: thumb_path

    };
    
    write_mov_img_to_file(mov_img_info.clone(), index.clone());
    println!("{:#?}", mov_img_info.clone());
    
   
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
    true
}

fn write_mov_img_to_file(movstrct: MovPosterInfo, idx: i32) {
    let mii = serde_json::to_string(&movstrct).unwrap();

    let fire_nfos_path =
        env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");

    let a = format!("{}/", fire_nfos_path.as_str());
    let b = format!("Movie_Image_Info_{}.json", idx.to_string());
    let outpath = a + &b;
    fs::write(outpath, &mii).expect("Failed to write named incorrectly json file");
}
