use crate::db::domain::auth::{JwtToken::JwtToken, UserObject::*};
use bcrypt::hash;
use bcrypt::{verify, DEFAULT_COST};
use diesel::insert_into;
use diesel::prelude::*;
use dotenv::dotenv;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, Validation, DecodingKey};
use std::{env, string};
use std::error::Error;

use super::domain::auth::UserObject::UserQuery;

pub struct Database {
    connection: PgConnection,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Database { connection: PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url)) }
    }

    pub fn authenticate(&mut self,
        auth_username: &str,
        auth_password: &str,
    ) -> Result<JwtToken, Box<dyn Error>> {
        use crate::schema::users::dsl::*;
        let user = users
            .filter(username.eq(auth_username))
            .first::<User>(&self.connection)
            .expect("Error while logging in");


        if verify(auth_password, &user.password).unwrap() {
            let identified_user = IdentifiedUser {
                username: user.username,
                email: user.email,
                exp: 86400,
            };

            let header = Header::new(Algorithm::HS512);
            let token = encode(
                &header,
                &identified_user,
                &EncodingKey::from_secret("8Zz5tw0Ionm3XPZZfN0NOml3z9FMfmpgXwovR9fp6ryDIoGRM8EPHAB6iHsc0fb".as_ref()),
            )
            .map_err(|_| "Error generating JWT token")?;
            Ok(JwtToken { token })
        } else {
            Ok(JwtToken { token: (&"todo!()").to_string() })
        }
    }

    pub fn register(
        &mut self,
        inserted_username: &str,
        inserted_password: &str,
        inserted_email: &str,
    ) -> Result<(), Box<dyn Error>> {
        use crate::schema::users::dsl::*;
        let hashed_password = hash(&inserted_password, DEFAULT_COST).unwrap();
        
        let new_user = vec![NewUser {
            username: &inserted_username,
            email: &inserted_email,
            password: &hashed_password,
        }];

        let reg: usize = insert_into(users)
            .values(&new_user)
            .execute(&self.connection)
            .map_err(|_| "Error while registering user")?;
        Ok(())
    }

    pub fn me(&mut self, token: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let validation = Validation::new(jsonwebtoken::Algorithm::HS512);
        let decoding_key = DecodingKey::from_secret("8Zz5tw0Ionm3XPZZfN0NOml3z9FMfmpgXwovR9fp6ryDIoGRM8EPHAB6iHsc0fb".as_ref());
        let token_object = jsonwebtoken::decode::<IdentifiedUser>(&token, &decoding_key, &validation).expect("Error while decoding token");
        Ok(serde_json::to_string(&token_object.claims).unwrap())
    }
}
