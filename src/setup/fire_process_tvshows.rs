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
        let mut tv_cat = Vec::new();
        let ap = &self.apath.to_string();
        // let ap = apa;
        if ap.contains("AlienWorlds") {
            tv_cat.push("AlienWorlds");
        } else if ap.contains("AlteredCarbon") {
            tv_cat.push("AlteredCarbon");
        } else if ap.contains("Andor") {
            tv_cat.push("AlteredCarbon");
        } else if ap.contains("TheBadBatch") {
            tv_cat.push("TheBadBatch");
        } else if ap.contains("BlackKnight") {
            tv_cat.push("BlackKnight");
        } else if ap.contains("BobbaFett") {
            tv_cat.push("BobbaFett");
        } else if ap.contains("CowboyBebop") {
            tv_cat.push("CowboyBebop");
        } else if ap.contains("Discovery") {
            tv_cat.push("Discovery");
        } else if ap.contains("Enterprise") {
            tv_cat.push("Enterprise");
        } else if ap.contains("ForAllManKind") {
            tv_cat.push("ForAllManKind");
        } else if ap.contains("Foundations") {
            tv_cat.push("Foundations");
        } else if ap.contains("Halo") {
            tv_cat.push("Halo");
        } else if ap.contains("HFord1923") {
            tv_cat.push("HFord1923");
        } else if ap.contains("HouseOfTheDragon") {
            tv_cat.push("HouseOfTheDragon");
        } else if ap.contains("LostInSpace") {
            tv_cat.push("LostInSpace");
        } else if ap.contains("LowerDecks") {
            tv_cat.push("LowerDecks");
        } else if ap.contains("Mandelorian") {
            tv_cat.push("Mandelorian");
        } else if ap.contains("NextGeneration") {
            tv_cat.push("NextGeneration");
        } else if ap.contains("NightSky") {
            tv_cat.push("NightSky");
        } else if ap.contains("ObiWanKenobi") {
            tv_cat.push("ObiWanKenobi");
        } else if ap.contains("Orville") {
            tv_cat.push("Orville");
        } else if ap.contains("PrehistoricPlanet") {
            tv_cat.push("PrehistoricPlanet");
        } else if ap.contains("Picard") {
            tv_cat.push("Picard");
        } else if ap.contains("Prodigy") {
            tv_cat.push("Prodigy");
        } else if ap.contains("RaisedByWolves") {
            tv_cat.push("RaisedByWolves");
        } else if ap.contains("Reacher") {
            tv_cat.push("Reacher");
        } else if ap.contains("RingsOfPower") {
            tv_cat.push("RingsOfPower");
        } else if ap.contains("StrangeNewWorlds") {
            tv_cat.push("StrangeNewWorlds");
        } else if ap.contains("STTV") {
            tv_cat.push("STTV");
        } else if ap.contains("TalesOfTheJedi") {
            tv_cat.push("TalesOfTheJedi");
        } else if ap.contains("TheLastOfUs") {
            tv_cat.push("TheLastOfus");
        } else if ap.contains("Visions") {
            tv_cat.push("Visions");
        } else if ap.contains("Voyager") {
            tv_cat.push("Visions");
        } else if ap.contains("WheelOfTime") {
            tv_cat.push("WheelOfTime")
        } else {
            println!("Fuck I forgot a catagory")
        }

        println!("this is self: {:?}", &self.clone());

        println!("this is tv_cat: {:?}", tv_cat.clone());

        tv_cat[0].to_string()
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
