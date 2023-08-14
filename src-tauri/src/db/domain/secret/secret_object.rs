use crate::schema::secrets;
use diesel::{sql_types::*, BelongingToDsl};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Clone, Debug, AsChangeset)]
pub struct Secret {
    pub id: i32,
    pub userid: i32,
    pub title: String,
    pub content: String,
    pub created_at: SystemTime,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "secrets"]
pub struct NewSecret<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub userid: &'a i32,
}
