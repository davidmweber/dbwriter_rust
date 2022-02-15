use api::models;
use crate::{api, db};
use diesel::pg::PgConnection;
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_sample_by_id(sample_id: i64, conn: &PgConnection) -> Result<Option<models::Sample>, DbError> {
    use db::schema::samples::dsl::*;
    let s = samples
        .filter(id.eq(&sample_id))
        .first::<models::Sample>(conn)
        .optional()?;
    Ok(s)
}

// pub fn insert_sample(sample: models::NewSample) {
//     return;
// }