extern crate chrono;
#[macro_use]
extern crate diesel;
mod schema;

use dotenv::dotenv;
use std::env;

use serde::{Deserialize, Serialize};
use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, QueryDsl};
use schema::samples;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Our database model. It does everything through the traits it derives
/// so it is super easy to serialize, query etc
#[derive(Insertable, Queryable, Serialize, Deserialize, Clone)]
//#[table_name = "samples"]
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
async fn get_samples(_pool: web::Data<DbPool>) -> &'static str {
    "bla"
}

type DbError = Box<dyn std::error::Error + Send + Sync>;

fn find_sample_by_id(sample_id: i64, conn: &PgConnection) -> Result<Option<Sample>, DbError> {
    use schema::samples::dsl::*;
    let s = samples
        .filter(id.eq(&sample_id))
        .first::<Sample>(conn)
        .optional()?;
    Ok(s)
}

#[get("/samples/{sample_id}")]
async fn get_sample(
    pool: web::Data<DbPool>,
    sample_id: web::Path<i64>,
) -> Result<HttpResponse, Error> {
    let s_id = sample_id.into_inner();

    // Shunt this to a thread pool so it does not block here.
    let s = web::block(move || {
        let conn = pool.get()?;
        find_sample_by_id(s_id, &conn)
    }).await?.map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(s) = s {
        Ok(HttpResponse::Ok().json(s))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No sample found with id: {}", s_id));
        Ok(res)
    }
}

// #[post("/sample")]
// async fn write_samples(pool: web::Data<DbPool>, form: web::Json<Sample>) -> &'static str {
//     "bla"
// }

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

    // Drop any existing data
    let conn = pool.get().unwrap(); // Grab a separate connection for each iteration
    let _ = diesel::delete(samples::dsl::samples).execute(&conn);

    // Put some stuff into our database
    for i in 0..10 {
        let record = Sample {
            id: i,
            name: String::from("frobnicator_manifold_pressure"),
            timestamp: Utc::now().naive_utc(),
            v0: Some(i as f32),
            v1: Some((2 * i) as f32),
        };
        let res = diesel::insert_into(samples::dsl::samples)
            .values(record)
            .execute(&conn);
        assert_eq!(res, Ok(1)); // Will be Err(something) if there was a problem
    }
    //let results = sample.ilter

    // Start up the HTTP server, set up the routes and and just block on its completion
    HttpServer::new(move || {
        // This is a lambda function with zero arguments
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(get_samples)
            .service(get_sample)
        //.service(write_samples)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
