use std::fs;
use std::env;
use std::clone::Clone;
use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MovPosterInfo {
    id: String,
    path: String,
    dims: String,
    size: String,
    name: String,
    thumbpath: String,
    idx: String
}

fn create_movie_thumbnail(x: String) -> String {
    let foobar = crate::setup::fire_utils::FireUtils {
        apath: x.clone()
    };
    let fire_movie_metadata_path =
        env::var("FIRE_THUMBNAILS").expect("$FIRE_THUMBNAILS is not set");
    let old_fname = crate::setup::fire_utils::FireUtils::split_poster_name(&foobar);
    println!("old_fname {}", old_fname);
    println!("this is x {}", x.clone());
    let out_fsize = fire_movie_metadata_path + "/" + &old_fname;
    let img = image::open(&x).expect(&x);
    let thumbnail = img.resize(230, 345, image::imageops::FilterType::Lanczos3);
    thumbnail
        .save(out_fsize.clone())
        .expect("Saving image failed");

    out_fsize
}

fn write_mov_img_to_file(movstrct: MovPosterInfo, dims: i32) {
    let mii = serde_json::to_string(&movstrct).unwrap();

    let fire_nfos_path =
        env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");

    let a = format!("{}/", fire_nfos_path.as_str());
    let b = format!("Movie_Image_Info_{}.json", dims.to_string());
    let outpath = a + &b;
    fs::write(outpath, &mii).expect("Failed to write sized incorrectly json file");
}

pub fn process_movie_posters(x: String, index: i32) -> i32 {
    // let movie_posters_vec = crate::fire_walk_dirs::walk_posters2_dir();
    // let mut index = 0;
    // let mut bad_image_vec = vec![];

    // for x in movie_posters_vec {
    // index = index + 1;

    let dims = crate::setup::fire_image::get_image_dims(&x);
    let dims_foo = format!("{:?}", dims);

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
        thumbpath: thumb_path,
        idx: index.to_string()


    };
    // println!("\n{:?}\n", mov_img_info.clone());
    write_mov_img_to_file(mov_img_info.clone(), index.clone());
    write_movie_images_to_db(mov_img_info).expect("movies image db insertion failed");
    
    index
}

fn write_movie_images_to_db(mov_img_info: MovPosterInfo) -> Result<()> {
    let conn = Connection::open("fire.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS movies_images (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL,
            dims TEXT NOT NULL,
            size TEXT NOT NULL,
            name TEXT NOT NULL,
            thumbpath TEXT NOT NULL,
            idx TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "INSERT INTO movies_images (
                path, 
                dims,
                size,
                name,
                thumbpath,
                idx
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            &mov_img_info.path,
            &mov_img_info.dims,
            &mov_img_info.size,
            &mov_img_info.name,
            &mov_img_info.thumbpath,
            &mov_img_info.idx
        ),
    )?;

    Ok(())
}


