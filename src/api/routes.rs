use actix_web::{get, web, Error, HttpResponse};
use crate::db::DbPool;
use crate::db::dao;

/// Simple return to test if the API is alive
#[get("/")]
pub async fn hello() -> &'static str {
    "Hello world!"
}

/// Returns all the samples in the database
#[get("/samples")]
pub async fn get_samples(_pool: web::Data<DbPool>) -> &'static str {
    "bla"
}

#[get("/samples/{sample_id}")]
pub async fn get_sample(
    pool: web::Data<DbPool>,
    sample_id: web::Path<i64>,
) -> Result<HttpResponse, Error> {
    let s_id = sample_id.into_inner();

    // Shunt this to a thread pool so it does not block here.
    let s = web::block(move || {
        let conn = pool.get()?;
        dao::find_sample_by_id(s_id, &conn)
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
// pub async fn write_samples(pool: web::Data<DbPool>, form: web::Json<Sample>) -> &'static str {
//     "bla"
// }
