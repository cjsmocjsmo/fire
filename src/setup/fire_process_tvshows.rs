use std::env;
use std::clone::Clone;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TVShowsStruc {
    id: String,
    fireid: String,
    idx: String,
    category: String,
    name: String,
    season: String,
    episode: String,
    size: String,
    httppath: String,
    vidtype: String,
}

#[derive(Debug)]
pub struct TVShowsUtils {
    apath: String,
}

impl TVShowsUtils {
    fn get_tv_category(&self) -> String {
        let ap = &self.apath.to_string();
        match true {
            true if ap.contains("AlienWorlds") => String::from("AlienWorlds"),
            true if ap.contains("AlteredCarbon") => String::from("AlteredCarbon"),
            true if ap.contains("Andor") => String::from("Andor"),
            true if ap.contains("TheBadBatch") => String::from("TheBadBatch"),
            true if ap.contains("BlackKnight") => String::from("BlackKnight"),
            true if ap.contains("BobbaFett") => String::from("BobbaFett"),
            true if ap.contains("CowboyBebop") => String::from("CowboyBebop"),
            true if ap.contains("Discovery") => String::from("Discovery"),
            true if ap.contains("Enterprise") => String::from("Enterprise"),
            true if ap.contains("ForAllManKind") => String::from("ForAllManKind"),
            true if ap.contains("Foundations") => String::from("Foundations"),
            true if ap.contains("Halo") => String::from("Halo"),
            true if ap.contains("HFord1923") => String::from("HFord1923"),
            true if ap.contains("HouseOfTheDragon") => String::from("HouseOfTheDragon"),
            true if ap.contains("LostInSpace") => String::from("LostInSpace"),
            true if ap.contains("LowerDecks") => String::from("LowerDecks"),
            true if ap.contains("Mandelorian") => String::from("Mandelorian"),
            true if ap.contains("NextGeneration") => String::from("NextGeneration"),
            true if ap.contains("NightSky") => String::from("NightSky"),
            true if ap.contains("ObiWanKenobi") => String::from("ObiWanKenobi"),
            true if ap.contains("Orville") => String::from("Orville"),
            true if ap.contains("PrehistoricPlanet") => String::from("PrehistoricPlanet"),
            true if ap.contains("Picard") => String::from("Picard"),
            true if ap.contains("Prodigy") => String::from("Prodigy"),
            true if ap.contains("RaisedByWolves") => String::from("RaisedByWolves"),
            true if ap.contains("Reacher") => String::from("Reacher"),
            true if ap.contains("RingsOfPower") => String::from("RingsOfPower"),
            true if ap.contains("StrangeNewWorlds") => String::from("StrangeNewWorlds"),
            true if ap.contains("StarTrek") => String::from("StarTrek"),
            true if ap.contains("StarWars") => String::from("shit"), //fake movies
            true if ap.contains("TalesOfTheJedi") => String::from("TalesOfTheJedi"),
            true if ap.contains("TheLastOfUs") => String::from("TheLastOfus"),
            true if ap.contains("Visions") => String::from("Visions"),
            true if ap.contains("Voyager") => String::from("Visions"),
            true if ap.contains("WheelOfTime") => String::from("WheelOfTime"),
            _ => String::from("Fuck Me")
        }
    }


    // fn get_tv_category(&self) -> String {
    //     let foo12 = crate::setup::fire_utils::FireUtils {
    //         apath: self.apath.to_string(),
    //     };

    //     let name = crate::setup::fire_utils::FireUtils::split_movie_name(&foo12);
    //     let n_split = name.split(" ");
    //     let mut n_split_vec = vec![];

    //     for n in n_split {
    //         n_split_vec.push(n);
    //     }

    //     let idx = n_split_vec.len() - 2;
    //     let mut newname_vec = vec![];
    //     let foo = n_split_vec.drain(0..idx);

    //     for f in foo {
    //         newname_vec.push(f);
    //     }

    //     let bar = newname_vec.join(" ");

    //     bar.to_string()
    // }

    fn get_tv_episode_season(&self) -> (String, String) {
        let foo1 = crate::setup::fire_utils::FireUtils {
            apath: self.apath.to_string(),
        };

        let name = crate::setup::fire_utils::FireUtils::split_movie_name(&foo1);
        let n_split = name.split(" ");
        let mut n_split_vec = vec![];

        for n in n_split {
            n_split_vec.push(n);
        }

        let idx = &n_split_vec.len() - 2;
        let parts: Vec<char> = n_split_vec[idx.clone()].chars().collect();
        let season = parts[1].to_string() + &parts[2].to_string();
        let episode = parts[4].to_string() + &parts[5].to_string();
        let results = (season, episode);

        results
    }
}

fn write_tvshows_nfos(tvs: TVShowsStruc, count: i32) {
    let tvshows_info = serde_json::to_string(&tvs).unwrap();
    let fire_nfo_path = env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");
    let a = format!("{}/", fire_nfo_path.as_str());
    let b = format!("TVShows_Meta_{}.json", count.to_string());
    let outpath = a + &b;
    std::fs::write(outpath, tvshows_info).unwrap();
}

pub fn process_tvshows(tvshows_vec: Vec<String>) -> bool {
    let mut count = 0;
    for tv in tvshows_vec {
        if tv.contains("TVShows") {
            count = count + 1;
            let tvshows = crate::setup::fire_utils::FireUtils { apath: tv.clone() };
            let tvshows2 = crate::setup::fire_process_tvshows::TVShowsUtils { apath: tv.clone() };
            let file_size = crate::setup::fire_utils::FireUtils::get_file_size(&tvshows);
            let category =
                crate::setup::fire_process_tvshows::TVShowsUtils::get_tv_category(&tvshows2);
            let es =
                crate::setup::fire_process_tvshows::TVShowsUtils::get_tv_episode_season(&tvshows2);
            let fire_id = crate::setup::fire_utils::FireUtils::get_md5(&tvshows);
            let fname = crate::setup::fire_utils::FireUtils::split_filename(&tvshows);
            let mut fnsplit_vec = Vec::new();
            let fnsplit = fname.split(" ");
            for f in fnsplit {
                fnsplit_vec.push(f.clone());
            }
            let episodename = fnsplit_vec.pop().unwrap();
            let tvshows = TVShowsStruc {
                id: count.clone().to_string(),
                fireid: fire_id,
                idx: count.clone().to_string(),
                category: category.clone(),
                name: episodename.clone().to_string(),
                season: es.0,
                episode: es.1,
                size: file_size,
                httppath: tv,
                vidtype: String::from("tvshow")
            };
            println!("this is tvshows \n\t{:?}", tvshows);
            write_tvshows_nfos(tvshows.clone(), count);
            write_tvshow_to_db(tvshows.clone()).expect("tvshows write to db failed");
        }
    }
    true
}

fn write_tvshow_to_db(tvs: TVShowsStruc) -> Result<()> {
    let conn = Connection::open("fire.db").unwrap();

    conn.execute("DROP TABLE IF EXISTS tvshows;", ())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tvshows (
            id INTEGER PRIMARY KEY,
            fireid TEXT NOT NULL,
            idx TEXT NOT NULL,
            category TEXT NOT NULL,
            name TEXT NOT NULL,
            season TEXT NOT NULL,
            episode TEXT NOT NULL,
            size TEXT NOT NULL,
            httppath TEXT NOT NULL,
            vidtype TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "INSERT INTO tvshows (
                fireid, 
                idx, 
                category, 
                name, 
                season, 
                episode, 
                size, 
                httppath,
                vidtype
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            &tvs.fireid,
            &tvs.idx,
            &tvs.category,
            &tvs.name,
            &tvs.season,
            &tvs.episode,
            &tvs.size,
            &tvs.httppath,
            &tvs.vidtype,
        ),
    )?;

    Ok(())
}
