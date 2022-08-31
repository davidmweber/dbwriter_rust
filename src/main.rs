#[macro_use]
extern crate diesel;
use dotenv::dotenv;
use std::env;
mod api;
mod db;

use actix_web::{web, App, HttpServer};
use api::*;
use chrono::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Set up a connection pool to the database
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::get_db_pool(db_url);
    // Drop any existing data and set up some sample data
    {
        let conn = &mut pool.get().unwrap(); // Grab a separate connection for each iteration
        db::dao::delete_all_samples(conn);

        // Put some stuff into our database
        for i in 0..10 {
            let record = models::Sample {
                id: i,
                name: String::from("frobnicator_manifold_pressure"),
                timestamp: Utc::now().naive_utc(),
                v0: Some(i as f32),
                v1: Some((2 * i) as f32),
            };
            db::dao::insert_sample(conn, record);
        }
    }
    // Start up the HTTP server, set up the routes and and just block on its completion
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Database dependency
            .configure(api::config_app())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
