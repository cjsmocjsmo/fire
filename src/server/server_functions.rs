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

#[get("/movies/action")]
async fn action() -> impl Responder  {
    let action_mov = crate::movies::action::action_movs().await.unwrap();
    HttpResponse::Ok().json(action_mov)
}

#[get("/movies/arnold")]
async fn arnold() -> impl Responder  {
    let arnold_mov = crate::movies::arnold::arnold_movs().await.unwrap();
    HttpResponse::Ok().json(arnold_mov)
}

#[get("/movies/brucewillis")]
async fn brucewillis() -> impl Responder  {
    let brucewillis_mov = crate::movies::brucewillis::brucewillis_movs().await.unwrap();
    HttpResponse::Ok().json(brucewillis_mov)
}

#[get("/movies/cartoons")]
async fn cartoons() -> impl Responder  {
    let cartoon_mov = crate::movies::cartoons::cartoons_movs().await.unwrap();
    HttpResponse::Ok().json(cartoon_mov)
}

#[get("/movies/comedy")]
async fn comedy() -> impl Responder  {
    let comedy_mov = crate::movies::comedy::comedy_movs().await.unwrap();
    HttpResponse::Ok().json(comedy_mov)
}

#[get("/movies/documentary")]
async fn documentary() -> impl Responder  {
    let documentary_mov = crate::movies::documentary::documentary_movs().await.unwrap();
    HttpResponse::Ok().json(documentary_mov)
}

#[get("/movies/drama")]
async fn drama() -> impl Responder  {
    let drama_mov = crate::movies::drama::drama_movs().await.unwrap();
    HttpResponse::Ok().json(drama_mov)
}

#[get("/movies/fantasy")]
async fn fantasy() -> impl Responder  {
    let fantasy_mov = crate::movies::fantasy::fantasy_movs().await.unwrap();
    HttpResponse::Ok().json(fantasy_mov)
}

#[get("/movies/godzilla")]
async fn godzilla() -> impl Responder  {
    let godzilla_mov = crate::movies::godzilla::godzilla_movs().await.unwrap();
    HttpResponse::Ok().json(godzilla_mov)
}

#[get("/movies/harrypotter")]
async fn harrypotter() -> impl Responder  {
    let harrypotter_mov = crate::movies::harrypotter::harrypotter_movs().await.unwrap();
    HttpResponse::Ok().json(harrypotter_mov)
}

#[get("/movies/indianajones")]
async fn indianajones() -> impl Responder  {
    let indy_mov = crate::movies::indianajones::indianajones_movs().await.unwrap();
    HttpResponse::Ok().json(indy_mov)
}

#[get("/movies/jamesbond")]
async fn jamesbond() -> impl Responder  {
    let jamesbond_mov = crate::movies::jamesbond::jamesbond_movs().await.unwrap();
    HttpResponse::Ok().json(jamesbond_mov)
}

#[get("/movies/johnwayne")]
async fn johnwayne() -> impl Responder  {
    let johnwayne_mov = crate::movies::johnwayne::johnwayne_movs().await.unwrap();
    HttpResponse::Ok().json(johnwayne_mov)
}

#[get("/movies/johnwick")]
async fn johnwick() -> impl Responder  {
    let johnwick_mov = crate::movies::johnwick::johnwick_movs().await.unwrap();
    HttpResponse::Ok().json(johnwick_mov)
}

#[get("/movies/jurassicpark")]
async fn jurassicpark() -> impl Responder  {
    let jurassicpark_mov = crate::movies::jurassicpark::jurassicpark_movs().await.unwrap();
    HttpResponse::Ok().json(jurassicpark_mov)
}

#[get("/movies/kingsmen")]
async fn kingsmen() -> impl Responder  {
    let kingsmen_mov = crate::movies::kingsmen::kingsmen_movs().await.unwrap();
    HttpResponse::Ok().json(kingsmen_mov)
}

#[get("/movies/meninblack")]
async fn meninblack() -> impl Responder  {
    let meninblack_mov = crate::movies::meninblack::meninblack_movs().await.unwrap();
    HttpResponse::Ok().json(meninblack_mov)
}

#[get("/movies/misc")]
async fn misc() -> impl Responder  {
    let misc_mov = crate::movies::misc::misc_movs().await.unwrap();
    HttpResponse::Ok().json(misc_mov)
}

#[get("/movies/nicolascage")]
async fn nicolascage() -> impl Responder  {
    let nicolascage_mov = crate::movies::nicolascage::nicolascage_movs().await.unwrap();
    HttpResponse::Ok().json(nicolascage_mov)
}

#[get("/movies/pirates")]
async fn pirates() -> impl Responder  {
    let pirates_mov = crate::movies::pirates::pirates_movs().await.unwrap();
    HttpResponse::Ok().json(pirates_mov)
}

#[get("/movies/riddick")]
async fn riddick() -> impl Responder  {
    let riddick_mov = crate::movies::riddick::riddick_movs().await.unwrap();
    HttpResponse::Ok().json(riddick_mov)
}

#[get("/movies/startreck")]
async fn startreck() -> impl Responder  {
    let startrek_mov = crate::movies::startreck::startreck_movs().await.unwrap();
    HttpResponse::Ok().json(startrek_mov)
}

#[get("/movies/starwars")]
async fn starwars() -> impl Responder  {
    let starwars_mov = crate::movies::starwars::starwars_movs().await.unwrap();
    HttpResponse::Ok().json(starwars_mov)
}

#[get("/movies/superheros")]
async fn superheros() -> impl Responder  {
    let superheroes_mov = crate::movies::superheros::superheros_movs().await.unwrap();
    HttpResponse::Ok().json(superheroes_mov)
}

#[get("/movies/scifi")]
async fn scifi() -> impl Responder  {
    let scifi_mov = crate::movies::scifi::scifi_movs().await.unwrap();
    HttpResponse::Ok().json(scifi_mov)
}

#[get("/movies/tomcruize")]
async fn tomcruize() -> impl Responder  {
    let tomcruize_mov = crate::movies::tomcruize::tomcruize_movs().await.unwrap();
    HttpResponse::Ok().json(tomcruize_mov)
}

#[get("/movies/transformers")]
async fn transformers() -> impl Responder  {
    let transformers_mov = crate::movies::transformers::transformers_movs().await.unwrap();
    HttpResponse::Ok().json(transformers_mov)
}

#[get("/movies/tremors")]
async fn tremors() -> impl Responder  {
    let tremors_mov = crate::movies::tremors::tremors_movs().await.unwrap();
    HttpResponse::Ok().json(tremors_mov)
}

#[get("/movies/therock")]
async fn therock() -> impl Responder  {
    let therock_mov = crate::movies::therock::therock_movs().await.unwrap();
    HttpResponse::Ok().json(therock_mov)
}

#[get("/movies/xmen")]
async fn xmen() -> impl Responder  {
    let xmen_mov = crate::movies::xmen::xmen_movs().await.unwrap();
    HttpResponse::Ok().json(xmen_mov)
}
