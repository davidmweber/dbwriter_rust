
#[macro_use]
extern crate diesel;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
pub mod schema;
use schema::sample;
// use schema::sample::dsl::sample;
use diesel::pg::PgConnection;
// use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::{Connection, RunQueryDsl};

#[derive(Insertable)]
#[table_name = "sample"]
pub struct AddSample<'a> {
    name: &'a str,
    timestamp: Option<&'a str>,
    // I really don't know what &'a means in this context
    v0: &'a Option<&'a f64>,
    v1: &'a Option<&'a f64>,
}

fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to database at {}", db_url));
    for i in 0..10 {
        let record = AddSample {
            name: "floof",
            timestamp: None,
            v0: &Some(&(i as f64)),
            v1: &Some(&((2 * i) as f64))
        };
        let inserts = diesel::insert_into(sample::dsl::sample)
            .values(record)
            .execute(&connection);
    }
}
