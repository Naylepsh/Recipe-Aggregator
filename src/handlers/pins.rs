use actix_web::{error, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::pin::Pin;
use crate::models::pin::NewPin;
use crate::models::pin::PinList;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub async fn index(
  tmpl: web::Data<tera::Tera>,
  pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
  let pin_list = PinList::list(pool);

  let mut ctx = tera::Context::new();
  ctx.insert("pins", &pin_list);
  let s = tmpl.render("pins/index.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show(
  id: web::Path<String>,
  tmpl: web::Data<tera::Tera>,
  pool: web::Data<Pool>)
-> Result<HttpResponse, Error> {
  let pin = web::block(move || Pin::find(id.into_inner(), pool))
  .await
  .map_err(|_| HttpResponse::InternalServerError())?;

  let mut ctx = tera::Context::new();
  ctx.insert("pin", &pin);
  let s = tmpl.render("pins/show.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn new(
  tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
  let s = tmpl.render("pins/new.html", &tera::Context::new())
              .map_err(|_| error::ErrorInternalServerError("Template error"))?;
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn create(
  tmpl: web::Data<tera::Tera>,
  params: web::Form<NewPin>,
  pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
  let pool_backup = pool.clone();
  web::block(move || params.create(pool))
  .await
  .map_err(|_| HttpResponse::InternalServerError())?;

  index(tmpl, pool_backup).await
}

pub async fn edit(
  id: web::Path<String>,
  tmpl: web::Data<tera::Tera>,
  pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
  let pin = web::block(move || Pin::find(id.into_inner(), pool))
  .await
  .map_err(|_| HttpResponse::InternalServerError())?;

  let mut ctx = tera::Context::new();
  ctx.insert("pin", &pin);
  let s = tmpl.render("pins/edit.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
  Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn update(
  id: web::Path<String>,
  tmpl: web::Data<tera::Tera>,
  params: web::Form<NewPin>,
  pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
  let id_backup = web::Path::from(id.clone());
  let pool_backup = pool.clone();
  web::block(move || Pin::update(id.into_inner(), params.into_inner(), pool))
  .await
  .map_err(|_| HttpResponse::InternalServerError())?;

  show(id_backup, tmpl, pool_backup).await
}

pub async fn destroy(
  id: web::Path<String>,
  tmpl: web::Data<tera::Tera>,
  pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
  let pool_backup = pool.clone();
  web::block(move || Pin::destroy(id.into_inner(), pool))
  .await
  .map_err(|_| HttpResponse::InternalServerError())?;

  index(tmpl, pool_backup).await
}