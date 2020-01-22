use actix_web::{error, web, Error, HttpResponse, HttpRequest};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::pin::Pin;
use crate::models::pin::NewPin;
use crate::models::pin::PinList;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub async fn index(tmpl: web::Data<tera::Tera>, pool: web::Data<Pool>, _req: HttpRequest) -> Result<HttpResponse, Error> {
  let pin_list = PinList::list(pool);
  Ok(HttpResponse::Ok().json(pin_list))
  // Ok(web::block(move || PinList::list(pool))
  // .await
  // .map(|pins| HttpResponse::Ok().json(pins))
  // .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn new_post(tmpl: web::Data<tera::Tera>, _req: HttpRequest) -> Result<HttpResponse, Error> {
  let s = tmpl.render("new-post.html", &tera::Context::new())
              .map_err(|_| error::ErrorInternalServerError("Template error"))?;
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn create(params: web::Form<NewPin>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
  // add actual front-end instead of json
  Ok(web::block(move || params.create(pool))
  .await
  .map(|pin| HttpResponse::Ok().json(pin))
  .map_err(|_| HttpResponse::InternalServerError())?)
}