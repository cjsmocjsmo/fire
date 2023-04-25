use json::object;
use std::env;
use std::path::Path;

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

pub fn process_movies() -> String {
    let movies_vec = crate::mtv_walk_dirs::walk_movies_dir();
    let mut count = 0;
    for x in movies_vec {
        count = count + 1;

        let mov_name = crate::mtv_split::split_movie_name(x.clone());
        let mov_year = crate::mtv_split::split_movie_year(x.clone());
        let mov_poster_addr = get_poster_addr(x.clone());
        let mov_size = crate::mtv_misc::get_file_size(&x);
        let mov_file_exists = Path::new(&mov_poster_addr).exists();

        let mov_js_obj = object! {
            name: mov_name,
            year: mov_year,
            poster_addr: mov_poster_addr,
            size: mov_size,
            exists: mov_file_exists,
            path: x,
            index: count.to_string()
        };

        let json_info = json::stringify(mov_js_obj.dump());

        let mtv_movies_metadata_path =
            env::var("MTV_MOVIES_METADATA_PATH").expect("$MTV_MOVIES_METADATA_PATH is not set");

        let a = format!("{}/", mtv_movies_metadata_path.as_str());
        let b = format!("Movie_Meta_{}.json", &count);
        let outpath = a + &b;

        std::fs::write(outpath, json_info).unwrap();
    }

    count.to_string()
}
