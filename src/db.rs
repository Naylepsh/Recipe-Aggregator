use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
  let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
  let manager = ConnectionManager::<SqliteConnection>::new(connspec);
  let pool = r2d2::Pool::builder()
      .build(manager)
      .expect("Failed to create pool.");
  pool.clone()
}
