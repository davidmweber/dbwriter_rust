pub mod dao;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

// Grabs a connection pool given the database URL
pub fn get_db_pool(db_url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .max_size(12)
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
