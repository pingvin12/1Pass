use std::time::SystemTime;

use crate::schema::secrets;
use serde::Serialize;

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Secret {
    pub id: i32,
    pub uid: i32,
    pub name: String,
    pub value: String,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "secrets"]
pub struct NewSecret<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub uid: &'a i32,
}
