use crate::schema::users;
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Deserialize, Serialize, Ord, Eq, PartialEq, PartialOrd)]
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

#[derive(Debug, Deserialize, Serialize, QueryableByName)]
#[table_name = "users"]
pub struct UserQuery {
    #[sql_type = "Text"]
    pub username: String,
    #[sql_type = "Text"]
    pub email: String,
    #[sql_type = "Text"]
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IdentifiedUser {
    pub username: String,
    pub email: String,
    pub userid: i32,
    pub exp: usize,
}
