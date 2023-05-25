use actix_files as fs;
use actix_web::{web, App, HttpServer};
use std::env;

pub mod server_functions;

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
            .service(crate::server::server_functions::comedy)
            .service(crate::server::server_functions::documentary)
            .service(crate::server::server_functions::drama)
            .service(crate::server::server_functions::fantasy)
            .service(crate::server::server_functions::godzilla)
            .service(crate::server::server_functions::harrypotter)
            .service(crate::server::server_functions::indianajones)
            .service(crate::server::server_functions::jamesbond)
            .service(crate::server::server_functions::johnwayne)
            .service(crate::server::server_functions::johnwick)
            .service(crate::server::server_functions::jurassicpark)
            .service(crate::server::server_functions::kingsmen)
            .service(crate::server::server_functions::meninblack)
            .service(crate::server::server_functions::misc)
            .service(crate::server::server_functions::nicolascage)
            .service(crate::server::server_functions::pirates)
            .service(crate::server::server_functions::riddick)
            .service(crate::server::server_functions::startreck)
            .service(crate::server::server_functions::starwars)
            .service(crate::server::server_functions::superheros)
            .service(crate::server::server_functions::scifi)
            .service(crate::server::server_functions::tomcruize)
            .service(crate::server::server_functions::transformers)
            .service(crate::server::server_functions::tremors)
            .service(crate::server::server_functions::therock)
            .service(crate::server::server_functions::xmen)
            .service(fs::Files::new("/thumbnails", img_path.clone()).show_files_listing())
            .route(
                "/hey",
                web::get().to(crate::server::server_functions::manual_hello),
            )
    })
    .bind(("192.168.0.26", 8080))?
    .run()
    .await
}
