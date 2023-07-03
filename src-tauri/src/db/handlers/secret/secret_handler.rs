use std::{error::Error};
use crate::{schema::secrets::dsl::*, db::encryption::{encrypt_data, decrypt_data}};
use diesel::{insert_into, QueryDsl, RunQueryDsl, update};

use crate::{db::{database::Database, domain::secret::secret_object::{Secret, NewSecret}}, schema::secrets::{id}};
use crate::diesel::ExpressionMethods;
pub trait SecretManager {
    fn get_secret(&mut self, secret_id: &i32) -> Result<Secret, Box<dyn Error>>;
    fn delete_secret(&mut self, secret_id: &i32) -> Result<usize, Box<dyn Error>>;
    fn create_secret(
        &mut self,
        inserted_name: &str,
        inserted_content: &str,
        inserted_uid: &i32
    ) -> Result<usize, Box<dyn Error>>;
    fn query_secrets(&mut self, obj_uid: &i32) -> Result<Vec<Secret>, Box<dyn Error>>;
    fn update_secret(&mut self, secret_obj: Secret) -> Result<usize, Box<dyn Error>>;
}

impl SecretManager for Database {
    fn get_secret(&mut self, secret_id: &i32) -> Result<Secret, Box<dyn Error>> {
        let result = secrets.filter(id.eq(secret_id)).first::<Secret>(&mut self.connection)?;
        Ok(result)
    }

    fn delete_secret(&mut self, secret_id: &i32) -> Result<usize, Box<dyn Error>> {
        Ok(diesel::delete(secrets.filter(id.eq(secret_id)))
            .execute(&self.connection)? as usize)
    }

    fn create_secret(
        &mut self,
        inserted_name: &str,
        inserted_content: &str,
        inserted_uid: &i32
    ) -> Result<usize, Box<dyn Error>> {
        use crate::schema::secrets::dsl::*;
        let new_secret = NewSecret {
            title: &encrypt_data(inserted_name.to_owned()).to_owned(),
            content: &encrypt_data(inserted_content.to_owned()).to_owned(),
            userid: inserted_uid,
        };

        Ok(insert_into(secrets)
            .values(&new_secret)
            .execute(&self.connection)? as usize)
    }

    fn query_secrets(&mut self, obj_userid: &i32) -> Result<Vec<Secret>, Box<dyn Error>> {
        let secrets_query: Vec<Secret> = secrets.order_by(title.asc())
        .filter(userid.eq(obj_userid)).get_results(&mut self.connection)?;
    
        Ok(secrets_query.iter().map(|secret| {
            Secret {
                id: secret.id,
                title: decrypt_data(secret.title.to_owned()),
                content: decrypt_data(secret.content.to_owned()),
                userid: secret.userid,
                created_at: secret.created_at,
            }
        }).collect::<Vec<Secret>>())
    }

    fn update_secret(&mut self, secret_obj: Secret) -> Result<usize, Box<dyn Error>> {
        Ok(update(secrets.filter(id.eq(secret_obj.id)))
        .set(&secret_obj)
        .execute(&mut self.connection)? as usize)
    }
}