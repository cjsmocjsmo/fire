use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::env;
// use std::path::Path;

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
    println!("{:?}", p2_split);
    let mut p2_split_vec = vec![];
    for p in p2_split {
        p2_split_vec.push(p);
    }

    let poster_addr = p1.to_string() + &"Posters2/".to_string() + p2_split_vec[1];

    poster_addr
}

fn write_mov_meta_to_file(mi: MovieInfoStruc, count: i32) {
    let json_info = serde_json::to_string(&mi).unwrap();
    let fire_movies_metadata_path = env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");
    let a = format!("{}/", fire_movies_metadata_path.as_str());
    let b = format!("Movie_Meta_{}.json", count);
    let outpath = a + &b;
    std::fs::write(outpath, json_info).unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MovieInfoStruc {
    id: String,
    fireid: String,
    idx: String,
    name: String,
    year: String,
    size: String,
    httpposterpath: String,
    httpmoviepath: String,
}

pub fn process_movies(movies_vec: Vec<String> ) -> String {
    let mut count = 0;
    for x in movies_vec {
        if x.clone().contains("Movies") || x.clone().contains("movies") {

        
            count = count + 1;
            let foo = crate::setup::fire_utils::FireUtils { apath: x.clone() };
            let fire_id = crate::setup::fire_utils::FireUtils::get_md5(&foo);
            let mov_name = crate::setup::fire_utils::FireUtils::split_movie_name(&foo);
            let mov_size = crate::setup::fire_utils::FireUtils::get_file_size(&foo);
            
            let mov_year = crate::setup::fire_utils::FireUtils::split_movie_year(&foo);
            let mov_poster_addr = get_poster_addr(x.clone());
            
            println!("{}", fire_id.clone());
            println!("{}", mov_name.clone());
            println!("{}", mov_size.clone());
            println!("{}", mov_year.clone());
            println!("{}", mov_poster_addr);
            // let mov_info = MovieInfoStruc {
            //     id: count.clone().to_string(),
            //     fireid: fire_id,
            //     idx: count.clone().to_string(),
            //     name: mov_name,
            //     year: "1999".to_string(),
            //     size: mov_size,
            //     httpposterpath: mov_poster_addr.clone(),
            //     httpmoviepath: mov_poster_addr.clone(),
            // };
            // println!("{:?}", mov_info.clone());
            // write_mov_meta_to_file(mov_info.clone(), count.clone());
            // write_movies_to_db(mov_info.clone()).expect("movies db insert has failed");
        }
    }

    count.to_string()
}

fn write_movies_to_db(mov_info: MovieInfoStruc) -> Result<()> {
    let conn = Connection::open("fire.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS movies (
            id INTEGER PRIMARY KEY,
            fireid TEXT NOT NULL,
            idx TEXT NOT NULL,
            name TEXT NOT NULL,
            year TEXT NOT NULL,
            size TEXT NOT NULL,
            httpposterpath TEXT NOT NULL,
            httpmoviepath TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "INSERT INTO movies (
                fireid, 
                idx,
                name,
                year,
                size,
                httposterpath,
                httpmoviepath
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            &mov_info.fireid,
            &mov_info.idx,
            &mov_info.name,
            &mov_info.year,
            &mov_info.size,
            &mov_info.httpposterpath,
            &mov_info.httpmoviepath
        ),
    )?;

    Ok(())
}

