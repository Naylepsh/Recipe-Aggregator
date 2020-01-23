use serde::{Serialize, Deserialize};
use crate::schema::pins;
use crate::schema::pins::dsl::*;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use actix_web::web;
use diesel::r2d2::{self, ConnectionManager};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Insertable, Serialize, Deserialize, Queryable)]
pub struct Pin {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: String,
    pub source: String
}

impl Pin {
  pub fn new(uuid: String, pin_params: NewPin) -> Pin {
    Pin {
      id: uuid,
      title: pin_params.title,
      description: pin_params.description,
      image: pin_params.image,
      source: pin_params.source
    }
  }

  pub fn find(_id: String, pool: web::Data<Pool>) -> Result<Pin, diesel::result::Error> {
    let connection: &SqliteConnection = &pool.get().unwrap();
    pins::table.find(_id).first(connection)
  }
}

#[derive(Insertable, Deserialize)]
#[table_name = "pins"]
pub struct NewPin {
  pub id: Option<String>,
  pub title: Option<String>,
  pub description: Option<String>,
  pub image: String,
  pub source: String
}

impl NewPin {
  pub fn create(&self, pool: web::Data<Pool>) -> Result<Pin, diesel::result::Error> {
    let connection: &SqliteConnection = &pool.get().unwrap();
    let uuid = format!("{}", uuid::Uuid::new_v4());
    let x = uuid.clone();
    
    let pin = Pin {
      id: uuid,
      title: self.title.clone(),
      description: self.description.clone(),
      image: self.image.clone(),
      source: self.source.clone()
    };
    
    diesel::insert_into(pins::table)
      .values(pin)
      .execute(connection)?;
    
    let mut items = pins.filter(id.eq(x)).load::<Pin>(connection)?;
    Ok(items.pop().unwrap())
  }
}


#[derive(Serialize, Deserialize)] 
pub struct PinList(pub Vec<Pin>);

impl PinList {
  pub fn list(pool: web::Data<Pool>) -> Self {
    let conn: &SqliteConnection = &pool.get().unwrap();

    let result = 
      pins
        .limit(10)
        .load::<Pin>(conn)
        .expect("Error loading products");

    PinList(result)
  }
}
