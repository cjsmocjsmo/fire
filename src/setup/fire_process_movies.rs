use json::object;
use std::env;
use std::path::Path;
use serde::{Serialize, Deserialize};

fn get_poster_addr(x: String) -> String {
    let no_ext_name_res = x.split(".");
    let mut no_ext_name_vec = vec![];

    for n in no_ext_name_res {
        no_ext_name_vec.push(n);
    }

    let new_jpg_name = no_ext_name_vec[0].to_owned() + ".jpg";
    let new_jpg_name_split = new_jpg_name.split("Movies");
    let mut new_jpg_name_split_vec = vec![];

    for jpg in new_jpg_name_split {
        new_jpg_name_split_vec.push(jpg);
    }
    let p1 = new_jpg_name_split_vec[0];
    let p2 = new_jpg_name_split_vec[1];
    let p2_split = p2.split("/");
    let mut p2_split_vec = vec![];
    for p in p2_split {
        p2_split_vec.push(p);
    }

    let poster_addr = p1.to_string() + &"Posters2/".to_string() + p2_split_vec[2];

    poster_addr
}

#[derive(Serialize, Deserialize, Debug)]
struct MovieInfoStruc {
    id: String,
    fireid: String,
    index: String,
    name: String,
    year: String,
    size: String,
    httpposterpath: String,
    httpmoviepath: String
}

pub fn process_movies() -> String {
    let mut count = 0;
    for x in movies_vec {
        count = count + 1;

        let foo = crate::setup::fire_utils::FireUtils {
            apath: x
        };

        let fire_id = crate::setup::fire_utils::FireUtils::get_md5(&x);

        let mov_name = crate::setup::fire_utils::FireUtils::split_movie_name(&x);
        let mov_year = crate::setup::fire_utils::FireUtils::split_movie_year(&x);
        let mov_poster_addr = get_poster_addr(&x);
        let mov_size = crate::setup::fire_utils::FireUtils::get_file_size(&x);
        let fire_id = crate::setup::fire_utils::FireUtils::get_md5(&x);
        // let mov_file_exists = Path::new(&mov_poster_addr).exists();

        // let mov_js_obj = object! {
        //     fireid: ,
        //     index: count.to_string(),
        //     name: mov_name,
        //     year: mov_year,
        //     size: mov_size,
        //     httpposterpath: mov_poster_addr,
        //     httpmoviepath: mov_poster_addr
        // };
        
        // let json_info = json::stringify(mov_js_obj.dump());
        let mov_js_obj = MovieInfoStruc {
            id: count.clone().to_string(),
            fireid: fire_id,
            index: count.clone().to_string(),
            name: mov_name,
            year: mov_year,
            size: mov_size,
            httpposterpath: mov_poster_addr,
            httpmoviepath: mov_poster_addr
        };

        let json_info = serde_json::to_string(mov_js_obj).unwrap();

        let fire_movies_metadata_path =
            env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");

        let a = format!("{}/", fire_movies_metadata_path.as_str());
        let b = format!("Movie_Meta_{}.json", &count);
        let outpath = a + &b;

        std::fs::write(outpath, json_info).unwrap();
    }

    count.to_string()
}
