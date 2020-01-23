// Load modules
pub mod handlers;
pub mod db;
pub mod schema;
pub mod models;

use actix_web::{web, middleware, App, HttpServer};
use tera::Tera;

#[macro_use]
extern crate diesel;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .data(tera)
            .data(db::establish_connection()) // connect to database
            .wrap(middleware::Logger::default()) // enable logger
            .service(web::resource("/").route(web::get().to(handlers::landing::index)))
            .service(web::resource("/pins/new").route(web::get().to(handlers::pins::new)))
            .service(web::resource("/pins")
                .route(web::get().to(handlers::pins::index))
                .route(web::post().to(handlers::pins::create)))
            .service(web::resource("/pins/{id}")
                .route(web::get().to(handlers::pins::show)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}