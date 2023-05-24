use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MovInfoSt {
    id: i32,
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

pub async fn misc_movs() -> Result<Vec<MovInfoSt>> {
    let db = Connection::open("fire.db")?;
    let mut stmt  = db.prepare("
        SELECT id, fireid, idx, name, year, size, httpposterpath, path, category, vidtype FROM movies
        WHERE category = 'Misc';
    ")?;
    
    let action_movs_iter = stmt.query_map([], |row| {
        Ok(MovInfoSt {
            id: row.get(0)?,
            fireid: row.get(1)?,
            idx: row.get(2)?,
            name: row.get(3)?,
            year: row.get(4)?,
            size: row.get(5)?,
            httpposterpath: row.get(6)?,
            path: row.get(7)?,
            category: row.get(8)?,
            vidtype: row.get(9)?,
        })
    })?;

    let mut mov_vec = Vec::new();

    for mov in action_movs_iter {
        let m = mov.unwrap();
        mov_vec.push(m);
    }

    println!("{:?}", mov_vec);


    Ok(mov_vec)
}