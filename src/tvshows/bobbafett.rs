use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct TvShowsStruc {
    id: i32,
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

fn bobba_fett_season_1() -> Result<Vec<TvShowsStruc>> {
    let db = Connection::open("fire.db")?;
    let mut tv_vec = Vec::new();
    let mut stmt1  = db.prepare("
        SELECT id, fireid, idx, category, name, season, episode, size, httppath, vidtype FROM tvshows
        WHERE name='Bobba Fett', season='01'
    ")?;
    let bobba_fett_tv_iter = stmt1.query_map([], |row| {
        Ok(TvShowsStruc {
            id: row.get(0)?,
            fireid: row.get(1)?,
            idx: row.get(2)?,
            category: row.get(3)?,
            name: row.get(4)?,
            season: row.get(5)?,
            episode: row.get(6)?,
            size: row.get(7)?,
            httppath: row.get(8)?,
            vidtype: row.get(9)?,
        })
    })?;

    for mo in bobba_fett_tv_iter {
        let m = mo.unwrap();
        tv_vec.push(m);
    }

    println!("{:?}", tv_vec);

    Ok(tv_vec)
}

pub async fn bobba_fett_tv(season: String) -> Result<Vec<Vec<TvShowsStruc>>> {
    let mut epilist = Vec::new();

    if season == "1" {
        let zlist = bobba_fett_season_1();
        if let Ok(z) = zlist {
            epilist.push(z);
        }
    };
        
    Ok(epilist)
}
