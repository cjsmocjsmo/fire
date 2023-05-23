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

#[get("/indianajones")]
async fn indianajones() -> impl Responder  {
    let indy_mov = crate::server::indianajones::indianajones_movs().await.unwrap();
    HttpResponse::Ok().json(indy_mov)
}

