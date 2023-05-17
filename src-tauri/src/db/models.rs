use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_on: String,
}


#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct RegisterUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}