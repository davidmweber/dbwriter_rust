
#[macro_use]
extern crate diesel;
extern crate chrono;

pub mod schema;
use schema::sample;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::{Connection, RunQueryDsl};
use chrono::prelude::*;

#[derive(Debug, Insertable)]
#[table_name = "sample"]
pub struct AddSample<'a > {
    name: &'a str,
    timestamp: NaiveDateTime,
    v0: f32,
    v1: f32,
}


fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to database at {}", db_url));
    for i in 0..10 {
        let record = AddSample {
            name: "floof",
            timestamp: Utc::now().naive_utc(),
            v0: i as f32,
            v1: (2 * i) as f32
        };
        let inserts = diesel::insert_into(sample::dsl::sample)
            .values(record)
            .execute(&connection);
    }
}
