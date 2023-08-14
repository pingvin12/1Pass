use crate::db::domain::auth::{jwt_token::JwtToken, user_object::*};
use crate::db::domain::news;
use crate::utils::config::CONFIG;
use bcrypt::hash;
use bcrypt::{verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use dotenv::dotenv;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use r2d2_postgres::r2d2::Pool;
use r2d2_postgres::r2d2;

use r2d2_diesel::ConnectionManager;
use std::env;
use std::error::Error;
use tauri::AppHandle;

pub type DBPool = Pool<r2d2_diesel::ConnectionManager<PgConnection>>;
pub type DBConnection =
    r2d2_postgres::r2d2::PooledConnection<r2d2_diesel::ConnectionManager<PgConnection>>;
pub type DBManager = ConnectionManager<PgConnection>;
pub struct Database {
    pub conn: DBConnection,
}

impl Database {
    fn get_database_url() -> String {
        CONFIG.connection.clone()
    }

    pub fn new() -> Self {
        let pool = Self::new_pool().unwrap();
        let conn: DBConnection = pool.get().unwrap();
        Database { conn }
    }

    pub fn new_pool(
    ) -> Result<Pool<r2d2_diesel::ConnectionManager<PgConnection>>, r2d2_postgres::r2d2::Error>
    {
        let database_url = Self::get_database_url();
        let manager: DBManager = ConnectionManager::<PgConnection>::new(database_url);

        r2d2::Pool::new(manager)
    }

    pub fn authenticate(
        &mut self,
        auth_email: &str,
        auth_password: &str,
    ) -> Result<JwtToken, String> {
        use crate::schema::users::dsl::*;
        let user: User = match users
            .filter(email.eq(auth_email))
            .first::<User>(&mut *self.conn)
        {
            Ok(res) => res,
            Err(_err) => {
                error!("Error while authenticating user");
                return Err("Error while authenticating user".to_owned());
            }
        };

        if verify(auth_password, &user.password).unwrap() {
            let identified_user = IdentifiedUser {
                username: user.username,
                email: user.email,
                userid: user.id,
                exp: (Utc::now() + Duration::seconds(86400))
                    .timestamp()
                    .try_into()
                    .unwrap(),
            };

            let header = Header::new(Algorithm::HS512);
            let token = encode(
                &header,
                &identified_user,
                &EncodingKey::from_secret(
                    "8Zz5tw0Ionm3XPZZfN0NOml3z9FMfmpgXwovR9fp6ryDIoGRM8EPHAB6iHsc0fb".as_ref(),
                ), //placeholder
            )
            .map_err(|_| "Error generating JWT token")?;
            Ok(JwtToken { token })
        } else {
            Ok(JwtToken {
                token: (&"todo!()").to_string(),
            })
        }
    }

    pub fn register(
        &mut self,
        inserted_username: &str,
        inserted_password: &str,
        inserted_email: &str,
    ) -> Result<String, Box<dyn Error>> {
        use crate::schema::users::dsl::*;
        let hashed_password = hash(&inserted_password, DEFAULT_COST).unwrap();

        let new_user = vec![NewUser {
            username: &inserted_username,
            email: &inserted_email,
            password: &hashed_password,
        }];

        let reg: usize = insert_into(users)
            .values(&new_user)
            .execute(&mut *self.conn)
            .map_err(|_| "Error while registering user")?;
        Ok(reg.to_string())
    }

    pub fn me(&mut self, token: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let token_object = match jsonwebtoken::decode::<IdentifiedUser>(
            &token,
            &DecodingKey::from_secret(
                "8Zz5tw0Ionm3XPZZfN0NOml3z9FMfmpgXwovR9fp6ryDIoGRM8EPHAB6iHsc0fb".as_ref(),
            ),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(token) => token,
            Err(_) => {
                error!("Error while decoding token");
                return Ok("".to_string());
            }
        };

        Ok(serde_json::to_string(&token_object.claims).unwrap())
    }

    pub fn clean_server(&mut self) {
        use crate::schema::{news::dsl::*, secrets::dsl::*, users::dsl::*};
        diesel::delete(users).execute(&mut *self.conn).unwrap();
        diesel::delete(news).execute(&mut *self.conn).unwrap();
        diesel::delete(secrets).execute(&mut *self.conn).unwrap();
    }
}
