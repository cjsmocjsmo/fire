use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::env;

fn get_poster_addr(mname: String, myear: String) -> String {
    let addr = env::var("FIRE_HTTP_ADDR").unwrap().to_string();
    let port = env::var("FIRE_HTTP_PORT").unwrap().to_string();
    let poster_addr = addr + &port + &"/thumbnails/".to_string() + &mname + " (" + &myear + ").jpg";
    println!("poster_addr \n\t{}", poster_addr.clone());

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

fn mov_category(x: String) -> String {
    match true {
        true if x.contains("Action") => String::from("Action"),
        true if x.contains("Arnold") => String::from("Arnold"),
        true if x.contains("BruceWillis") => String::from("BruceWillis"),
        true if x.contains("Cartoons") => String::from("Cartoons"),
        true if x.contains("Comedy") => String::from("Comedy"),
        true if x.contains("Documentary") => String::from("Documentary"),
        true if x.contains("Drama") => String::from("Drama"),
        true if x.contains("Fantasy") => String::from("Fantasy"),
        true if x.contains("Godzilla") => String::from("Godzilla"),
        true if x.contains("HarryPotter") => String::from("HarryPotter"),
        true if x.contains("IndianaJones") => String::from("IndianaJones"),
        true if x.contains("JamesBond") => String::from("JamesBond"),
        true if x.contains("JohnWayne") => String::from("JohnWayne"),
        true if x.contains("JohnWick") => String::from("JohnWick"),
        true if x.contains("JurassicPark") => String::from("JurassicPark"),
        true if x.contains("KingMen") => String::from("KingMen"),
        true if x.contains("MenInBlack") => String::from("MenInBlack"),
        true if x.contains("Misc") => String::from("Misc"),
        true if x.contains("NicolasCage") => String::from("NicolasCage"),
        true if x.contains("Pirates") => String::from("Pirates"),
        true if x.contains("Riddick") => String::from("Riddick"),
        true if x.contains("SciFi") => String::from("SciFi"),
        true if x.contains("StarTrek") => String::from("StarTrek"),
        true if x.contains("StarWars") => String::from("StarWars"),
        true if x.contains("SuperHeroes") => String::from("SuperHeroes"),
        true if x.contains("TheRock") => String::from("TheRock"),
        true if x.contains("TomCruize") => String::from("TomCruize"),
        true if x.contains("Transformers") => String::from("Transformers"),
        true if x.contains("Tremors") => String::from("Tremors"),
        true if x.contains("XMen") => String::from("XMen"),
        _ => String::from("Fuck")
    }
}

// fn mov_category(x: String) -> String{
//     let mut mov_category = String::new();
//     if x.contains("Action") {
//         mov_category = String::from("Action");
//     } else if x.contains("Arnold") {
//         mov_category = String::from("Arnold");
//     } else if x.contains("BruceWillis") {
//         mov_category = String::from("BruceWillis");
//     } else if x.contains("Cartoons") {
//         mov_category = String::from("Cartoons");
//     } else if x.contains("Comedy") {
//         mov_category = String::from("Comedy");
//     } else if x.contains("Drama") {
//         mov_category = String::from("Drama");
//     } else if x.contains("Documentary") {
//         mov_category = String::from("Documentary");
//     } else if x.contains("Fantasy") {
//         mov_category = String::from("Fantasy");
//     } else if x.contains("Godzilla") {
//         mov_category = String::from("Godzilla");
//     } else if x.contains("HarryPotter") {
//         mov_category = String::from("HarryPotter");
//     } else if x.contains("IndianaJones") {
//         mov_category = String::from("IndianaJones");
//     } else if x.contains("JamesBond") {
//         mov_category = String::from("JamesBond");
//     } else if x.contains("JohnWayne") {
//         mov_category = String::from("JohnWayne");
//     } else if x.contains("JohnWick") {
//         mov_category = String::from("JohnWick");
//     } else if x.contains("JurassicPark") {
//         mov_category = String::from("JurassicPark");
//     } else if x.contains("KingsMen") {
//         mov_category = String::from("KingsMen");
//     } else if x.contains("MenInBlack") {
//         mov_category = String::from("MenInBlack");
//     } else if x.contains("Misc") {
//         mov_category = String::from("Misc");
//     } else if x.contains("MicolasCage") {
//         mov_category = String::from("MicolasCage");
//     } else if x.contains("Pirates") {
//         mov_category = String::from("Pirates");
//     } else if x.contains("Riddick") {
//         mov_category = String::from("Riddick");
//     } else if x.contains("StarWars") {
//         mov_category = String::from("StarWars");
//     } else if x.contains("StarTrek") {
//         mov_category = String::from("Startreck");
//     } else if x.contains("SuperHeroes") {
//         mov_category = String::from("SuperHeroes");
//     } else if x.contains("SciFi") {
//         mov_category = String::from("SciFi");
//     } else if x.contains("TomCruize") {
//         mov_category = String::from("TomCruize");
//     } else if x.contains("Transformers") {
//         mov_category = String::from("Transformers");
//     } else if x.contains("Tremors") {
//         mov_category = String::from("Tremors");
//     } else if x.contains("TheRock") {
//         mov_category = String::from("TheRock");
//     } else if x.contains("XMen") {
//         mov_category = String::from("XMen");
//     };

//     mov_category
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MovieInfoStruc {
    // id: String,
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
            let mov_poster_addr = get_poster_addr(mov_name.clone(), mov_year.clone());
            let mov_info = MovieInfoStruc {
                // id: count.clone().to_string(),
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
            println!("\nthis is mov info \n{:?}\n", mov_info.clone());
            println!("{}", x.clone());
            write_mov_meta_to_file(mov_info.clone(), count.clone());
            write_movies_to_db(mov_info.clone()).expect("movies db insert has failed");
        }
    }

    count.to_string()
}

fn write_movies_to_db(mov_info: MovieInfoStruc) -> Result<()> {
    let conn = Connection::open("fire.db").unwrap();
    conn.execute("DROP TABLE IF EXISTS movies;", ())?;
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

