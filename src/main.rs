// Load modules
pub mod handlers;

use actix_web::{web, middleware, App, HttpServer};
use tera::Tera;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .data(tera)
            .wrap(middleware::Logger::default()) // enable logger
            .service(web::resource("/").route(web::get().to(handlers::landing::index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}