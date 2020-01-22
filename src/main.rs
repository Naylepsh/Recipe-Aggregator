// Load modules
pub mod handlers;

use actix_web::{web, App, HttpServer};
use tera::Tera;

// async fn index(tmpl: web::Data<tera::Tera>, _req: HttpRequest) -> Result<HttpResponse, Error> {
//     let s = tmpl.render("index.html", &tera::Context::new())
//                 .map_err(|_| error::ErrorInternalServerError("Template error"))?;
//     Ok(HttpResponse::Ok().content_type("text/html").body(s))
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    HttpServer::new(|| {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .data(tera)
            .service(web::resource("/").route(web::get().to(handlers::landing::index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}