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

#[derive(Serialize, Debug)]
struct MPI {
    id: i32,
    path: String,
    dims: String,
    size: String,
    name: String,
    thumbpath: String,
    idx: String
}

async fn get_action_movs() -> Result<Vec<MPI>> {
    let db = Connection::open("fire.db")?;
    let mut stmt  = db.prepare("
        SELECT id, path, dims, size, name, thumbpath, idx FROM movies_images
    ")?;
    let action_movs_iter = stmt.query_map([], |row| {
        Ok(MPI {
            id: row.get(0)?,
            path: row.get(1)?,
            dims: row.get(2)?,
            size: row.get(3)?,
            name: row.get(4)?,
            thumbpath: row.get(5)?,
            idx: row.get(6)?,
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