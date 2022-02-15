pub mod dao;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
