use dotenv::dotenv;
use std::env;
use diesel::associations::HasTable;
use diesel::insert_into;
use diesel::prelude::*;
use crate::db::models;
use super::{encryption::{self, encrypt_password, verify_password}};
use super::models::*;
use crate::schema::*;
use crate::schema::users::dsl::users;
use chrono::prelude::*;

struct Database {
    connection: PgConnection
}

struct JwtToken {
    token: String,
    validtill: i32,
}

impl Database {
    fn new() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
    }

    fn authenticate(conn: &mut PgConnection, username: &str, password: &str) -> JwtToken
    {
        JwtToken { token: "dsdfhisdg".to_string(), validtill: 86400 }
    }

    pub fn register(conn: &mut PgConnection, ins_username: String, ins_password: String, ins_email: String)
    {
        use crate::schema::users::dsl::*;
        let ins_user = vec![ NewUser { username: &ins_username, password: &ins_password, email: &ins_email }];
        //let new_user = insert_into(users::table).values(&ins_user).execute(conn);
    }


}

#[tauri::command]
pub async fn register_user(username: String, password: String, email: String)
{
    let mut connect = Database::new();
    let res = Database::register(&mut connect, username, password, email);
}

#[tauri::command]
pub async fn login_user(email: String, password: String)
{
    let mut connect = Database::new();
    let res = Database::authenticate(&mut connect, &email, &password);
}

