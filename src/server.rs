use actix_files as fs;
use actix_web::{web, App, HttpServer};
// use rusqlite::{Connection};
use std::env;

pub mod server_functions;
pub mod action;
pub mod arnold;
pub mod brucewillis;
pub mod cartoons;
pub mod indianajones;

#[actix_web::main]
pub async fn fire_server_main() -> std::io::Result<()> {
    let img_path = env::var("FIRE_THUMBNAILS").unwrap();

    HttpServer::new(move || {
        App::new()
            .service(crate::server::server_functions::hello)
            .service(crate::server::server_functions::echo)
            .service(crate::server::server_functions::action)
            .service(crate::server::server_functions::arnold)
            .service(crate::server::server_functions::brucewillis)

            .service(crate::server::server_functions::cartoons)
            .service(crate::server::server_functions::indianajones)
            .service(fs::Files::new("/thumbnails", img_path.clone()))
            // .service(fs::Files::new("/thumbnails", img_path.clone()).show_files_listing())
            .route(
                "/hey",
                web::get().to(crate::server::server_functions::manual_hello),
            )
    })
    .bind(("192.168.0.61", 8080))?
    .run()
    .await
}
