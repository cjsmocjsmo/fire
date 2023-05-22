use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::env;
// use std::path::Path;

fn get_poster_addr(x: String, mname: String) -> String {
    let no_ext_name_res = x.split(".");
    let mut no_ext_name_vec = vec![];

    for n in no_ext_name_res {
        no_ext_name_vec.push(n);
    }

    let new_jpg_name = no_ext_name_vec[0].to_owned() + ".jpg";
    println!("{}", new_jpg_name);
    // let new_jpg_name_split = new_jpg_name.split("Movies");
    // let mut new_jpg_name_split_vec = vec![];

    // for jpg in new_jpg_name_split {
    //     new_jpg_name_split_vec.push(jpg);
    // }
    // // let p1 = new_jpg_name_split_vec[0];
    // let p2 = new_jpg_name_split_vec[1];
    // let p2_split = p2.split("/");
    // let mut p2_split_vec = vec![];
    // for p in p2_split {
    //     p2_split_vec.push(p);
    // }

    // let hostaddr = gethostname();

    println!("{}", x.clone());
    let addr = env::var("FIRE_HTTP_ADDR").unwrap().to_string();
    let port = env::var("FIRE_HTTP_PORT").unwrap().to_string();

    let poster_addr = addr + &port + &"/thumbnails/".to_string() + &mname + ".jpg";

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

fn mov_category(x: String) -> String{
    let mut mov_category = String::new();
    if x.contains("Action") {
        mov_category = String::from("Action");
    } else if x.contains("Arnold") {
        mov_category = String::from("Arnold");
    } else if x.contains("BruceWillis") {
        mov_category = String::from("BruceWillis");
    } else if x.contains("Cartoons") {
        mov_category = String::from("Cartoons");
    } else if x.contains("Comedy") {
        mov_category = String::from("Comedy");
    } else if x.contains("Drama") {
        mov_category = String::from("Drama");
    } else if x.contains("Documentary") {
        mov_category = String::from("Documentary");
    } else if x.contains("Fantasy") {
        mov_category = String::from("Fantasy");
    } else if x.contains("Godzilla") {
        mov_category = String::from("Godzilla");
    } else if x.contains("HarryPotter") {
        mov_category = String::from("HarryPotter");
    } else if x.contains("IndianaJones") {
        mov_category = String::from("IndianaJones");
    } else if x.contains("JamesBond") {
        mov_category = String::from("JamesBond");
    } else if x.contains("JohnWayne") {
        mov_category = String::from("JohnWayne");
    } else if x.contains("JurassicPark") {
        mov_category = String::from("JurassicPark");
    } else if x.contains("KingsMen") {
        mov_category = String::from("KingsMen");
    } else if x.contains("MenInBlack") {
        mov_category = String::from("MenInBlack");
    } else if x.contains("Misc") {
        mov_category = String::from("Misc");
    } else if x.contains("MicolasCage") {
        mov_category = String::from("MicolasCage");
    } else if x.contains("Pirates") {
        mov_category = String::from("Pirates");
    } else if x.contains("Riddick") {
        mov_category = String::from("Riddick");
    } else if x.contains("StarWars") {
        mov_category = String::from("StarWars");
    } else if x.contains("StarTrek") {
        mov_category = String::from("StarTrek");
    } else if x.contains("SuperHeroes") {
        mov_category = String::from("SuperHeroes");
    } else if x.contains("SciFi") {
        mov_category = String::from("SciFi");
    } else if x.contains("TomCruize") {
        mov_category = String::from("TomCruize");
    } else if x.contains("Transformers") {
        mov_category = String::from("Transformers");
    } else if x.contains("Tremors") {
        mov_category = String::from("Tremors");
    } else if x.contains("TheRock") {
        mov_category = String::from("TheRock");
    } else if x.contains("XMen") {
        mov_category = String::from("XMen");
    };

    mov_category
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
    path: String,
    category: String,
    vidtype: String
}



pub fn process_movies(movies_vec: Vec<String> ) -> String {
    let mut count = 0;
    for x in movies_vec {
        if x.clone().contains("Movies") {
            count = count + 1;
            let foo = crate::setup::fire_utils::FireUtils { apath: x.clone() };
            let fire_id = crate::setup::fire_utils::FireUtils::get_md5(&foo);
            let mov_name = crate::setup::fire_utils::FireUtils::split_movie_name(&foo);
            let mov_size = crate::setup::fire_utils::FireUtils::get_file_size(&foo);
            let mov_year = crate::setup::fire_utils::FireUtils::split_movie_year(&foo);
            let mov_poster_addr = get_poster_addr(x.clone(), mov_name.clone());
            let mov_info = MovieInfoStruc {
                id: count.clone().to_string(),
                fireid: fire_id,
                idx: count.clone().to_string(),
                name: mov_name.clone(),
                year: mov_year,
                size: mov_size,
                httpposterpath: mov_poster_addr.clone(),
                path: x.clone(),
                category: mov_category(x.clone()),
                vidtype: String::from("Movie"),
            };
            println!("\n{:?}\n", mov_info.clone());
            write_mov_meta_to_file(mov_info.clone(), count.clone());
            write_movies_to_db(mov_info.clone()).expect("movies db insert has failed");
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
            path TEXT NOT NULL,
            category TEXT NOT NULL,
            vidtype TEXT NOT NULL
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
                httpposterpath,
                path,
                category,
                vidtype
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            &mov_info.fireid,
            &mov_info.idx,
            &mov_info.name,
            &mov_info.year,
            &mov_info.size,
            &mov_info.httpposterpath,
            &mov_info.path,
            &mov_info.category,
            &mov_info.vidtype
        ),
    )?;

    Ok(())
}

