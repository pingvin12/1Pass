use crate::db::domain::auth::{JwtToken::JwtToken, UserObject::NewUser, UserObject::User};
use crate::schema::users::dsl::users;
use crate::schema::*;
use bcrypt::hash;
use bcrypt::{verify, DEFAULT_COST};
use chrono::prelude::*;
use diesel::associations::HasTable;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::select;
use dotenv::dotenv;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::env;
use std::error::Error;

pub struct Database {
    connection: PgConnection,
}

impl Database {
    pub fn new() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    pub fn authenticate(
        conn: &mut PgConnection,
        username: &str,
        pwd: &str,
    ) -> Result<JwtToken, Box<dyn Error>> {
        use crate::schema::users::dsl::*;
        let mut user = users
            .filter(username.eq(username))
            .first::<User>(conn)
            .expect("Error while logging in");

        //verify(pwd, &user.password);
        //.map_err(|_| "Invalid u(sername or password".to_owned())?;
        if user.password.eq(pwd) {
            let token = encode(
                &Header::default(),
                &user,
                &EncodingKey::from_secret("your-secret-key".as_ref()),
            )
            .map_err(|_| "Error generating JWT token")?;
            Ok(JwtToken { token })
        } else {
            Ok(JwtToken {
                token: "".to_owned(),
            })
        }
    }

    pub fn register(
        conn: &PgConnection,
        ins_username: &str,
        ins_password: &str,
        ins_email: &str,
    ) -> Result<(), Box<dyn Error>> {
        use crate::schema::users::dsl::*;
        let hashpass = hash(&ins_password, DEFAULT_COST).unwrap();
        let new_user = vec![NewUser {
            username: &ins_username,
            email: &ins_email,
            password: &ins_password,
        }];

        let reg: usize = insert_into(users)
            .values(&new_user)
            .execute(conn)
            .map_err(|_| "Error while registering user")?;
        Ok(())
    }
}
