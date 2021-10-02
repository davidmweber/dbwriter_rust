#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod schema;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::schema::sample::columns::{name, timestamp};
use crate::schema::sample::dsl::sample;

fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to database at {}", db_url));

    let record = (&name.eq("foo"), &timestamp.eq()
    let inserts = diesel::insert_into(sample)
        .values(&name.eq("foo")).execute(&connection);
    print!("{} inserts", inserts.unwrap())
}
