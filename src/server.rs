use actix_web::{web, App, HttpServer};
use std::env;
use actix_files as fs;

pub mod server_functions;


// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
pub async fn fire_server_main() -> std::io::Result<()> {
    let img_path = env::var("FIRE_THUMBNAILS").unwrap();
    HttpServer::new(move || {
        App::new()
            .service(crate::server::server_functions::hello)
            .service(crate::server::server_functions::echo)
            .route("/hey", web::get().to(crate::server::server_functions::manual_hello))
            .service(fs::Files::new("/img", img_path.clone()).show_files_listing())
    })
    .bind(("192.168.0.61", 8080))?
    .run()
    .await
}