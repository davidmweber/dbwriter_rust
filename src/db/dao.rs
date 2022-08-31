use crate::{api, db};
use api::models;
use db::schema::samples::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::models::Success;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_sample_by_id(
    conn: &mut PgConnection,
    sample_id: i64,
) -> Result<Option<models::Sample>, DbError> {
    let s = samples
        .filter(id.eq(&sample_id))
        .first::<models::Sample>(conn)
        .optional()?;
    Ok(s)
}

pub fn insert_sample(conn: &mut PgConnection, sample: models::Sample) {
    let res = diesel::insert_into(samples).values(sample).execute(conn);
    assert_eq!(res, Ok(1));
}

// Careful now. This drops all records from the samples table
pub fn delete_all_samples(conn: &mut PgConnection) {
    let _ = diesel::delete(db::schema::samples::dsl::samples).execute(conn);
}

// Careful now. This drops all records from the samples table
pub fn delete_sample(conn: &mut PgConnection, sample_id: i64) -> Result<Success, DbError> {
    let _ = diesel::delete(db::schema::samples::dsl::samples)
        .filter(id.eq(sample_id))
        .execute(conn);
    return Ok(Success{});
}
