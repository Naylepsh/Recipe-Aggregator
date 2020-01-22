use actix_web::{error, web, Error, HttpResponse, HttpRequest};

pub async fn index(tmpl: web::Data<tera::Tera>, _req: HttpRequest) -> Result<HttpResponse, Error> {
  let s = tmpl.render("index.html", &tera::Context::new())
              .map_err(|_| error::ErrorInternalServerError("Template error"))?;
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}