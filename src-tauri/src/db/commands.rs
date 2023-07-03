use log::{error, info};

use crate::db::database::Database;

use super::{encryption::{generate_encryption_key, self}, handlers::secret::secret_handler::SecretManager, domain::auth::user_object::IdentifiedUser};

#[tauri::command]
pub fn register(username: String, password: String, email: String) -> Result<String, String> {
    let mut connection = Database::new();
    let res = Database::register(&mut connection, &username, &password, &email); // map err to string
    match res {
        Ok(result) => Ok(result),
        Err(_err) => {
            error!("Error while registering");
            return Ok("0".to_owned());
        }
    }
}

#[tauri::command]
pub fn auth(email: String, password: String) -> Result<String, String> {
    let mut connection = Database::new();
    let auth_response = match Database::authenticate(&mut connection, &email, &password) {
        Ok(result) => Ok(result.token),
        Err(_err) => {
            error!("Error while authenticating");
            return Err(_err.to_string());
        }
    };

    auth_response
}

#[tauri::command]
pub fn me(token: String) -> Result<String, String> {
    let mut connection = Database::new();
    let user_me = match Database::me(&mut connection, &token) {
        Ok(res) => Ok(res),
        Err(_err) => {
            error!("Error while getting user");
            return Ok(_err.to_string());
        }
    };
    user_me
}

#[tauri::command]
pub fn init_hosting() {
    let generated = generate_encryption_key(32);
    encryption::set_encryption_key(generated);
    info!("Encryption key generated");
}

#[tauri::command]
pub fn get_secrets(token: String) -> Result<String, String> {
    let mut connection = Database::new();

    let identified_user = match me(token) {
        Ok(res) => res,
        Err(_err) => {
            error!("Error while getting user");
            return Ok("[]".to_owned());
        }
    };

    let user = serde_json::from_str::<IdentifiedUser>(&identified_user).unwrap();

    let res = match <dyn SecretManager>::query_secrets(&mut connection, &user.userid) {
        Ok(res) => res,
        Err(_err) => {
            error!("Error while getting secrets");
            return Ok("[]".to_owned());
        }
    };
    Ok(serde_json::to_string(&res).unwrap())
}

#[tauri::command]
pub fn create_secret(token: String, name: String, content: String) -> Result<usize, String> {
    let mut connection = Database::new();

    let identified_user = match me(token) {
        Ok(res) => res,
        Err(_err) => {
            error!("Error while getting user");
            _err
        }
    };

    let user = serde_json::from_str::<IdentifiedUser>(&identified_user).unwrap();

    let secret = match <dyn SecretManager>::create_secret(&mut connection, &name, &content, &user.userid) {
        Ok(res) => res,
        Err(_err) => {
            error!("Error creating secret");
            return Ok(0);
        }
    };

    Ok(secret)
}

#[tauri::command]
pub fn delete_secret(id: i32) {
    let mut connection = Database::new();
    match <dyn SecretManager>::delete_secret(&mut connection, &id) {
        Ok(_res) => (),
        Err(_err) => {
            error!("Error deleting secret");
        }
    }
}

#[tauri::command]
pub fn clean_server() {
    let mut connection = Database::new();
    Database::clean_server(&mut connection);
}