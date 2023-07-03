use std::time::SystemTime;
use diesel::{sql_types::Text, query_dsl::LoadQuery};
use crate::schema::news;
use serde::{Serialize, Deserialize};

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