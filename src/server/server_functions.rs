use actix_web::{get, post, web, HttpResponse, Responder};
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


///////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////


#[get("/tvshows/alienworlds/{season}")]
async fn alienworlds(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let alienworlds_tv = crate::tvshows::alienworlds::alien_worlds_tv(season).await.unwrap();
    HttpResponse::Ok().json(alienworlds_tv)
}

#[get("/tvshows/alteredcarbon/{season}")]
async fn alteredcarbon(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let alteredcarbon_tv = crate::tvshows::alteredcarbon::altered_carbon_tv(season).await.unwrap();
    HttpResponse::Ok().json(alteredcarbon_tv)
}

#[get("/tvshows/andor/{season}")]
async fn andor(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let andor_tv = crate::tvshows::andor::andor_tv(season).await.unwrap();
    HttpResponse::Ok().json(andor_tv)
}

#[get("/tvshows/badbatch/{season}")]
async fn badbatch(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let badbatch_tv = crate::tvshows::badbatch::bad_batch_tv(season).await.unwrap();
    HttpResponse::Ok().json(badbatch_tv)
}

#[get("/tvshows/blackknight/{season}")]
async fn blackknight(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let blackknight_tv = crate::tvshows::blackknight::black_knight_tv(season).await.unwrap();
    HttpResponse::Ok().json(blackknight_tv)
}

#[get("/tvshows/bobbafett/{season}")]
async fn bobbafett(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let bobbafett_tv = crate::tvshows::bobbafett::bobba_fett_tv(season).await.unwrap();
    HttpResponse::Ok().json(bobbafett_tv)
}

#[get("/tvshows/cowboybebop/{season}")]
async fn cowboybebop(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let cowboybebop_tv = crate::tvshows::cowboybebop::cowboy_bebop_tv(season).await.unwrap();
    HttpResponse::Ok().json(cowboybebop_tv)
}

#[get("/tvshows/discovery/{season}")]
async fn discovery(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let discovery_tv = crate::tvshows::discovery::discovery_tv(season).await.unwrap();
    HttpResponse::Ok().json(discovery_tv)
}

#[get("/tvshows/enterprise/{season}")]
async fn enterprise(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let enterprise_tv = crate::tvshows::enterprise::enterprise_tv(season).await.unwrap();
    HttpResponse::Ok().json(enterprise_tv)
}

#[get("/tvshows/forallmankind/{season}")]
async fn forallmankind(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let forallmankind_tv = crate::tvshows::forallmankind::for_all_mankind_tv(season).await.unwrap();
    HttpResponse::Ok().json(forallmankind_tv)
}

#[get("/tvshows/foundations/{season}")]
async fn foundations(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let foundations_tv = crate::tvshows::foundations::foundations_tv(season).await.unwrap();
    HttpResponse::Ok().json(foundations_tv)
}

#[get("/tvshows/halo/{season}")]
async fn halo(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let halo_tv = crate::tvshows::halo::halo_tv(season).await.unwrap();
    HttpResponse::Ok().json(halo_tv)
}

#[get("/tvshows/hford1923/{season}")]
async fn hford1923(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let hford1923_tv = crate::tvshows::hford1923::hford1923_tv(season).await.unwrap();
    HttpResponse::Ok().json(hford1923_tv)
}

#[get("/tvshows/houseofthedragon/{season}")]
async fn houseofthedragon(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let houseofthedragon_tv = crate::tvshows::houseofthedragon::house_of_the_dragon_tv(season).await.unwrap();
    HttpResponse::Ok().json(houseofthedragon_tv)
}

#[get("/tvshows/lostinspace/{season}")]
async fn lostinspace(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let lostinspace_tv = crate::tvshows::lostinspace::lost_in_space_tv(season).await.unwrap();
    HttpResponse::Ok().json(lostinspace_tv)
}

#[get("/tvshows/lowerdecks/{season}")]
async fn lowerdecks(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let lowerdecks_tv = crate::tvshows::lowerdecks::lower_decks_tv(season).await.unwrap();
    HttpResponse::Ok().json(lowerdecks_tv)
}

#[get("/tvshows/mandalorian/{season}")]
async fn mandalorian(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let mandalorian_tv = crate::tvshows::mandalorian::mandalorian_tv(season).await.unwrap();
    HttpResponse::Ok().json(mandalorian_tv)
}

#[get("/tvshows/nextgeneration/{season}")]
async fn nextgeneration(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let nextgeneration_tv = crate::tvshows::nextgeneration::next_generation_tv(season).await.unwrap();
    HttpResponse::Ok().json(nextgeneration_tv)
}

#[get("/tvshows/nightsky/{season}")]
async fn nightsky(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let nightsky_tv = crate::tvshows::nightsky::night_sky_tv(season).await.unwrap();
    HttpResponse::Ok().json(nightsky_tv)
}

#[get("/tvshows/obiwan/{season}")]
async fn obiwan(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let obiwan_tv = crate::tvshows::obiwan::obiwan_tv(season).await.unwrap();
    HttpResponse::Ok().json(obiwan_tv)
}

#[get("/tvshows/orville/{season}")]
async fn orville(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let orville_tv = crate::tvshows::orville::orville_tv(season).await.unwrap();
    HttpResponse::Ok().json(orville_tv)
}

#[get("/tvshows/picard/{season}")]
async fn picard(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let picard_tv = crate::tvshows::picard::picard_tv(season).await.unwrap();
    HttpResponse::Ok().json(picard_tv)
}

#[get("/tvshows/prodigy/{season}")]
async fn prodigy(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let prodigy_tv = crate::tvshows::prodigy::prodigy_tv(season).await.unwrap();
    HttpResponse::Ok().json(prodigy_tv)
}

#[get("/tvshows/raisedbywolves/{season}")]
async fn raisedbywolves(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let raisedbywolves_tv = crate::tvshows::raisedbywolves::raised_by_wolves_tv(season).await.unwrap();
    HttpResponse::Ok().json(raisedbywolves_tv)
}

#[get("/tvshows/reacher/{season}")]
async fn reacher(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let reacher_tv = crate::tvshows::reacher::reacher_tv(season).await.unwrap();
    HttpResponse::Ok().json(reacher_tv)
}

#[get("/tvshows/ringsofpower/{season}")]
async fn ringsofpower(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let ringsofpower_tv = crate::tvshows::ringsofpower::rings_of_power_tv(season).await.unwrap();
    HttpResponse::Ok().json(ringsofpower_tv)
}

#[get("/tvshows/strangenewworlds/{season}")]
async fn strangenewworlds(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let strangenewworlds_tv = crate::tvshows::strangenewworlds::strange_new_worlds_tv(season).await.unwrap();
    HttpResponse::Ok().json(strangenewworlds_tv)
}

#[get("/tvshows/sttv/{season}")]
async fn sttv(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let sttv_tv = crate::tvshows::sttv::sttv_tv(season).await.unwrap();
    HttpResponse::Ok().json(sttv_tv)
}

#[get("/tvshows/talesofthejedi/{season}")]
async fn talesofthejedi(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let talesofthejedi_tv = crate::tvshows::talesofthejedi::tales_of_the_jedi_tv(season).await.unwrap();
    HttpResponse::Ok().json(talesofthejedi_tv)
}

#[get("/tvshows/thelastofus/{season}")]
async fn thelastofus(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let thelastofus_tv = crate::tvshows::thelastofus::the_last_of_us_tv(season).await.unwrap();
    HttpResponse::Ok().json(thelastofus_tv)
}

#[get("/tvshows/visions/{season}")]
async fn visions(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let visions_tv = crate::tvshows::visions::visions_tv(season).await.unwrap();
    HttpResponse::Ok().json(visions_tv)
}

#[get("/tvshows/voyager/{season}")]
async fn voyager(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let voyager_tv = crate::tvshows::voyager::voyager_tv(season).await.unwrap();
    HttpResponse::Ok().json(voyager_tv)
}

#[get("/tvshows/wheeloftime/{season}")]
async fn wheeloftime(path: web::Path<(String, String)>) -> impl Responder  {
    let (_, season) = path.into_inner();
    let wheeloftime_tv = crate::tvshows::wheeloftime::wheel_of_time_tv(season).await.unwrap();
    HttpResponse::Ok().json(wheeloftime_tv)
}