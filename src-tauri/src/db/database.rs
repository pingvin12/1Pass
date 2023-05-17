use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use super::{encryption::{self, encrypt_password, verify_password}};
use super::models::*;
use crate::schema::*;

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

    fn authenticate(username: String, password: String) //-> JwtToken
    {

    }

    pub fn register(conn: &mut PgConnection, username: &str, password: &str, email: &str) -> Result<(), &'static str>
    {
        let new_user = RegisterUser { username, email, password };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .expect("Error saving new post")
    }
}

#[tauri::command]
pub async fn register_user(username: String, password: String, email: String)
{
    let mut connect = Database::new();
    let res = Database::register(&mut connect, &username, &password, &email);
}