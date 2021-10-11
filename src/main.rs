extern crate chrono;
#[macro_use]
extern crate diesel;
mod schema;

use std::env;
use dotenv::dotenv;

use actix_web::{get, post, App, HttpServer, web, HttpResponse, Error};
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::{QueryDsl, r2d2};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use serde::{Deserialize, Serialize};
use schema::samples;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Our database model. It does everything through the traits it derives
/// so it is super easy to serialize, query etc
#[derive(Insertable, Queryable, Serialize, Deserialize, Clone)]
#[table_name = "samples"]
pub struct Sample {
    id: i64,
    name: String,
    timestamp: NaiveDateTime,
    v0: Option<f32>,
    v1: Option<f32>,
}

/// Simple return to test if the API is alive
#[get("/")]
async fn hello() -> &'static str {
    "Hello world!"
}

/// Returns all the samples in the database
#[get("/samples")]
async fn get_samples(pool: web::Data<DbPool>) -> &'static str {
    "bla"
}

#[get("/samples/{sample_id}")]
async fn get_sample(pool: web::Data<DbPool>, sample_id: web::Path<i64>)  -> Result<HttpResponse, Error> {
    use schema::samples::dsl::*;
    let s_id = sample_id.into_inner();
    let conn = pool.get().expect("Could not get a database connection from the pool");

    // TODO: Wrap this i a web::block() thingy
    let s = samples.filter(id.eq(&s_id))
        .first::<Sample>(&conn);
    match s {
        Ok(r) => Ok(HttpResponse::Ok().json(s)),
        Err(e) => Ok(HttpResponse::NotFound().body(format!("User not found {}", s_id)))
    }
}

#[post("/sample")]
async fn write_samples(pool: web::Data<DbPool>, form: web::Json<Sample>) -> &'static str {
    "bla"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Set up a connection pool to the database
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .max_size(12)
        .build(manager)
        .expect(&format!("Error connecting to database"));

    // Put some stuff into our database
    for i in 0..10 {
        let conn = pool.get().unwrap(); // Grab a separate connection for each iteration
        let record = Sample {
            name: String::from("frobnicator_manifold_pressure"),
            timestamp: Utc::now().naive_utc(),
            v0: i as f32,
            v1: (2 * i) as f32,
        };
        let res = diesel::insert_into(samples::dsl::samples)
            .values(record)
            .execute(&conn);
            assert_eq!(res, Ok(1)); // Will be Err(something) if there was a problem
    }
    //let results = sample.ilter

    // Start up the HTTP server, set up the routes and and just block on its completion
    HttpServer::new(move || {  // This is a lambda function with zero arguments
        App::new()
            .data(pool.clone())
            .service(hello)
            .service(get_samples)
            .service(get_sample)
            //.service(write_samples)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
