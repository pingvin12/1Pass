use std::time::SystemTime;

use serde::{Serialize};
use crate::schema::{secrets};

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