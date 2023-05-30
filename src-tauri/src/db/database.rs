use bcrypt::hash;
use diesel::select;
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
use bcrypt::{verify, DEFAULT_COST};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};


struct Database {
    connection: PgConnection
}

pub struct JwtToken {
    pub token: String,
    pub validtill: i32,
}

impl Database {
    fn new() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
    }

    fn authenticate(conn: &mut PgConnection, username: &str, pwd: &str) -> Result<JwtToken, String>
    {
        use crate::schema::users::dsl::*;
        let mut user = users.filter(username.eq(username))
        .first::<User>(conn)
        .expect("Error while logging in");

        //verify(pwd, &user.password);
        //.map_err(|_| "Invalid u(sername or password".to_owned())?;
        if(user.password.eq(pwd))
        {
            let token = encode(
                &Header::default(),
                &user,
                &EncodingKey::from_secret("your-secret-key".as_ref()),
            )
            .map_err(|_| "Error generating JWT token")?;
            Ok(JwtToken { token: token, validtill: 86400 })
        } else {
            Err("Bad username or password".to_owned())
        }
        }

    pub fn register(conn: &PgConnection, ins_username: String, ins_password: String, ins_email: String) -> Result<bool, String>
    {
        use crate::schema::users::dsl::*;
        let hashpass = hash(&ins_password, DEFAULT_COST).unwrap();
        let new_user = vec![ models::NewUser { username: &ins_username, email: &ins_email, password: &ins_password } ];
        
        let reg: usize = insert_into(users)
        .values(&new_user)
        .execute(conn).unwrap();
        if (reg > 0) {
            Ok(true)
        } else {
            Err("eError while registering".to_string())
        }
    }
}

pub fn register(username: String, password: String, email: String) -> Result<bool, String>
{
    let mut connect = Database::new();
    let res = Database::register(&mut connect, username, password, email);
    Ok(res.unwrap())
}

pub fn login(email: String, password: String) -> Result<JwtToken, String>
{
    let mut connect = Database::new();
    let res = Database::authenticate(&mut connect, &email, &password);
    match res {
        Ok(token) => Ok(token),
        Err(_err) => Err(_err),
    }
}

#[tauri::command]
pub fn command_register_user(username: String, password: String, email: String) -> Result<bool, String>
{
    let res = register(username, password, email); // map err to string
    match res {
        Ok(result) => Ok(result),
        Err(_err) => Err(_err),
    }
}

#[tauri::command]
pub fn command_login_user(email: String, password: String) -> Result<String, String>
{
    let res = login(email, password).unwrap();
    Ok(res.token)
}