use actix_web::{get, post, HttpResponse, Responder};
// use actix_web::web::Json;
use rusqlite::{Connection, Result};
use serde::Serialize;
// use anyhow::Error;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// #[derive(Serialize, Debug)]
// struct MPI {
//     id: i32,
//     path: String,
//     dims: String,
//     size: String,
//     name: String,
//     thumbpath: String,
//     idx: String
// }

#[derive(Serialize, Debug)]
struct MovInfoSt {
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

async fn get_action_movs() -> Result<Vec<MovInfoSt>> {
    let db = Connection::open("fire.db")?;
    // let category = String::from("Action");
    let mut stmt  = db.prepare("
        SELECT id, fireid, idx, name, year, size, httpposterpath, path, category, vidtype FROM movies
        WHERE category = 'Action';
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
            path: row.get(6)?,
            category: row.get(7)?,
            vidtype: row.get(8)?,
        })
    })?;

    let mut mov_vec = Vec::new();

    for mov in action_movs_iter {
        let m = mov.unwrap();
        mov_vec.push(m);
    }


    Ok(mov_vec)
}

#[get("/action")]
async fn action() -> impl Responder  {
    let act_mov = get_action_movs().await.unwrap();
    HttpResponse::Ok().json(act_mov)
    

}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}