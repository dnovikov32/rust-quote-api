#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod tinkoff;
pub mod share;

pub fn db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Environment variable DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}
