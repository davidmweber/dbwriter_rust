use serde::{Deserialize, Serialize};
//use diesel::prelude::*;
use chrono::prelude::*;
//#[macro_use]
use db::schema::samples;
use crate::db;

// Base sample model
#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
pub struct Sample {
    pub id: i64,
    pub name: String,
    pub timestamp: NaiveDateTime,
    pub v0: Option<f32>,
    pub v1: Option<f32>,
}

#[derive(Serialize)]
pub struct Success {}