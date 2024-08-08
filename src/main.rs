#[macro_use]
extern crate diesel;
use std::env;

use actix_web::{web, App, HttpServer};
use chrono::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;

use api::*;

mod api;
mod db;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Set up a connection pool to the database
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            let username = env::var("USERNAME").expect("USERNAME must be set in the environment");
            let password = env::var("PASSWORD").expect("PASSWORD must be set in the environment");
            let host = env::var("HOST").expect("HOST must be set in the environment");
            let port = env::var("PORT").expect("PORT must be set in the environment");
            format!("postgres://{}:{}@{}:{}/{}", username, password, host, port, "postgres")
            }
    };
    let pool = db::get_db_pool(db_url);
    // Drop any existing data and set up some sample data
    {
        let conn = &mut pool.get().unwrap(); // Grab a separate connection for each iteration

        // By default, the output is thrown out. If you want to redirect it to stdout, you
        // should call embedded_migrations::run_with_output.
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        println!("Migrations complete");

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
    // Start up the HTTP server, set up the routes and just block on its completion
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Database dependency
            .configure(api::config_app())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
