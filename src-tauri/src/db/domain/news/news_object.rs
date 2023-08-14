use crate::schema::news;
use diesel::{query_dsl::LoadQuery, sql_types::Text};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Ord, Eq, PartialEq, PartialOrd, Clone, Debug, AsChangeset, Queryable)]
#[table_name = "news"]
pub struct News {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: SystemTime,
}

#[derive(Insertable, Clone, Debug)]
#[table_name = "news"]
pub struct NewNews<'a> {
    pub title: &'a str,
    pub content: &'a str,
}
