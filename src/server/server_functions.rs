use actix_web::{get, post, HttpResponse, Responder};
// use actix_web::web::Json;
// use rusqlite::{Connection, Result};
// use serde::Serialize;
// use anyhow::Error;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/action")]
async fn action() -> impl Responder  {
    let action_mov = crate::server::action::action_movs().await.unwrap();
    HttpResponse::Ok().json(action_mov)
}

#[get("/arnold")]
async fn arnold() -> impl Responder  {
    let arnold_mov = crate::server::arnold::arnold_movs().await.unwrap();
    HttpResponse::Ok().json(arnold_mov)
}

#[get("/brucewillis")]
async fn brucewillis() -> impl Responder  {
    let brucewillis_mov = crate::server::brucewillis::brucewillis_movs().await.unwrap();
    HttpResponse::Ok().json(brucewillis_mov)
}

#[get("/cartoons")]
async fn cartoons() -> impl Responder  {
    let cartoon_mov = crate::server::cartoons::cartoons_movs().await.unwrap();
    HttpResponse::Ok().json(cartoon_mov)
}

#[get("/comedy")]
async fn comedy() -> impl Responder  {
    let comedy_mov = crate::server::comedy::comedy_movs().await.unwrap();
    HttpResponse::Ok().json(comedy_mov)
}

#[get("/documentary")]
async fn documentary() -> impl Responder  {
    let documentary_mov = crate::server::documentary::documentary_movs().await.unwrap();
    HttpResponse::Ok().json(documentary_mov)
}

#[get("/drama")]
async fn drama() -> impl Responder  {
    let drama_mov = crate::server::drama::drama_movs().await.unwrap();
    HttpResponse::Ok().json(drama_mov)
}

#[get("/fantasy")]
async fn fantasy() -> impl Responder  {
    let fantasy_mov = crate::server::fantasy::fantasy_movs().await.unwrap();
    HttpResponse::Ok().json(fantasy_mov)
}

#[get("/godzilla")]
async fn godzilla() -> impl Responder  {
    let godzilla_mov = crate::server::godzilla::godzilla_movs().await.unwrap();
    HttpResponse::Ok().json(godzilla_mov)
}

#[get("/harrypotter")]
async fn harrypotter() -> impl Responder  {
    let harrypotter_mov = crate::server::harrypotter::harrypotter_movs().await.unwrap();
    HttpResponse::Ok().json(harrypotter_mov)
}

#[get("/indianajones")]
async fn indianajones() -> impl Responder  {
    let indy_mov = crate::server::indianajones::indianajones_movs().await.unwrap();
    HttpResponse::Ok().json(indy_mov)
}

