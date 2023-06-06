use std::time::SystemTime;

use crate::schema::users;
use serde::Serialize;

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
