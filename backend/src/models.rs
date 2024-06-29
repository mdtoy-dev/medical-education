use crate::schema::{courses, users};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Identifiable)]
#[diesel(table_name = courses)]
pub struct Course {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub duration: Option<i32>,
    pub category: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = courses)]
pub struct NewCourse {
    pub title: String,
    pub description: String,
    pub duration: Option<i32>,
    pub category: Option<String>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}
