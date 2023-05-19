use std::env;
use std::clone::Clone;
use serde::{Serialize, Deserialize};
use rusqlite::{Connection, Result};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct TVShowsStruc {
    fireid: String,
    idx: String,
    catagory: String,
    name: String,
    season: String,
    episode: String,
    size: String,
    httppath: String
}

pub struct TVShowsUtils {
    apath: String
}

impl TVShowsUtils {
    fn get_tv_catagory(&self) -> String {

        let foo12 = crate::setup::fire_utils::FireUtils {
            apath: self.apath.to_string()
        };

        let name = crate::setup::fire_utils::FireUtils::split_movie_name(&foo12);
        let n_split = name.split(" ");
        let mut n_split_vec = vec![];

        for n in n_split {
            n_split_vec.push(n);
        }

        let idx = n_split_vec.len() - 2;

        let mut newname_vec = vec![];

        let foo = n_split_vec.drain(0..idx);

        for f in foo {
            newname_vec.push(f);
        }

        let bar = newname_vec.join(" ");

        bar.to_string()
    }
    fn get_tv_episode_season(&self) -> (String, String) {
        let foo1 = crate::setup::fire_utils::FireUtils {
            apath: self.apath.to_string()
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

    println!("{:#?}", tvshows_info);

    let fire_nfo_path =
        env::var("FIRE_NFOS").expect("$FIRE_NFOS is not set");

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
            let tvshows = crate::setup::fire_utils::FireUtils {
                apath: tv.clone()
            };
            let tvshows2 = crate::setup::fire_process_tvshows::TVShowsUtils {
                apath: tv.clone()
            };
            let file_size = crate::setup::fire_utils::FireUtils::get_file_size(&tvshows);
            let catagory = crate::setup::fire_process_tvshows::TVShowsUtils::get_tv_catagory(&tvshows2);
            let es = crate::setup::fire_process_tvshows::TVShowsUtils::get_tv_episode_season(&tvshows2);
            let fire_id = crate::setup::fire_utils::FireUtils::get_md5(&tvshows);
            let fname = crate::setup::fire_utils::FireUtils::split_filename(&tvshows);
            let mut fnsplit_vec = Vec::new();
            let fnsplit = fname.split(" ");
            for f in fnsplit {
                fnsplit_vec.push(f.clone());
            };
            let episodename = fnsplit_vec.pop().unwrap();
            let tvshows = TVShowsStruc {
                fireid: fire_id,
                idx: count.clone().to_string(),
                catagory: catagory.clone(),
                name: episodename.clone().to_string(),
                season: es.0,
                episode: es.1,
                size: file_size,
                httppath: tv
            };
            write_tvshows_nfos(tvshows.clone(), count);
            write_tvshow_to_db(tvshows.clone()).expect("tvshows write to db failed");

        }
    }
    true
}

fn write_tvshow_to_db(tvs: TVShowsStruc) -> Result<()> {
    let conn = Connection::open("fire.db").unwrap();
    conn.execute(
        "CREATE TABLE tvshows (
            id INTEGER PRIMARY KEY,
            fireid TEXT NOT NULL,
            idx TEXT NOT NULL,
            catagory TEXT NOT NULL,
            name TEXT NOT NULL,
            season TEXT NOT NULL,
            episode TEXT NOT NULL,
            size TEXT NOT NULL,
            httppath TEXT NOT NULL, 

        )",
        (),
    )?;

    conn.execute(
        "INSERT INTO tvshow (fireid, idx, catagory, name, season, episode, size, httppath)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&tvs.fireid, &tvs.idx, &tvs.catagory, &tvs.name, &tvs.season, &tvs.episode, &tvs.size, &tvs.httppath),
    )?;

    Ok(())
}

