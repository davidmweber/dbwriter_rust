#[macro_use]
extern crate diesel;
use dotenv::dotenv;
use std::env;
mod api;
mod db;

use actix_web::{web, App, HttpServer};
use api::*;
use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Set up a connection pool to the database
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .max_size(12)
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to database"));

    // Drop any existing data
    let conn = pool.get().unwrap(); // Grab a separate connection for each iteration
    let _ = diesel::delete(db::schema::samples::dsl::samples).execute(&conn);

    // Put some stuff into our database
    for i in 0..10 {
        let record = models::Sample {
            id: i,
            name: String::from("frobnicator_manifold_pressure"),
            timestamp: Utc::now().naive_utc(),
            v0: Some(i as f32),
            v1: Some((2 * i) as f32),
        };
        let res = diesel::insert_into(db::schema::samples::dsl::samples)
            .values(record)
            .execute(&conn);
        assert_eq!(res, Ok(1)); // Will be Err(something) if there was a problem
    }
    //let results = sample.ilter

    // Start up the HTTP server, set up the routes and and just block on its completion
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::config_app())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
