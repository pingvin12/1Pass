use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use diesel::{connection, PgConnection};
use log::{error, info};
use tauri::{api::path::data_dir, AppHandle, Manager, State};

use crate::{appstate::AppState, db::database::Database};

use super::{
    database::{DBConnection, DBPool},
    domain::auth::user_object::IdentifiedUser,
    encryption::{self, generate_encryption_key},
    handlers::secret::secret_handler::SecretManager,
};

#[tauri::command]
pub fn register(
    app_state: State<AppState>,
    username: String,
    password: String,
    email: String,
) -> Result<String, String> {
    let pool: DBPool = app_state
        .db_manager
        .lock()
        .unwrap()
        .clone()
        .expect("Error sending request");

    let connection = pool.get().unwrap();

    let res = Database::register(
        &mut Database { conn: connection },
        &username,
        &password,
        &email,
    ); // map err to string
    match res {
        Ok(result) => Ok(result),
        Err(_err) => {
            error!("Error while registering");
            return Ok("0".to_owned());
        }
    }
}

#[tauri::command]
pub fn auth(app_state: State<AppState>, email: String, password: String) -> Result<String, String> {
    let pool = app_state
        .db_manager
        .lock()
        .unwrap()
        .clone()
        .expect("Error sending request");

    let connection: DBConnection = pool.get().unwrap();
    let auth_response =
        match Database::authenticate(&mut Database { conn: connection }, &email, &password) {
            Ok(result) => Ok(result.token),
            Err(_err) => {
                error!("Error while authenticating");
                return Err(_err.to_string());
            }
        };

    auth_response
}

#[tauri::command]
pub fn me(app_state: State<AppState>, token: String) -> Result<String, String> {
    let pool = app_state
        .db_manager
        .lock()
        .unwrap()
        .clone()
        .expect("Error sending request");

    let connection: DBConnection = pool.get().unwrap();
    let user_me = match Database::me(&mut Database { conn: connection }, &token) {
        Ok(res) => Ok(res),
        Err(_err) => {
            error!("Error while getting user");
            return Ok(_err.to_string());
        }
    };
    user_me
}

pub fn me_cmd(token: String) -> Result<String, String> {
    let pool = Database::new_pool().unwrap();
    let connection: DBConnection = pool.get().unwrap();
    let user_me = match Database::me(&mut Database { conn: connection }, &token) {
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
pub fn get_secrets(app_state: State<AppState>, token: String) -> Result<String, String> {
    let pool = app_state
        .db_manager
        .lock()
        .unwrap()
        .clone()
        .expect("Error sending request");

    let connection: DBConnection = pool.get().unwrap();
    let identified_user = match me(app_state, token) {
        Ok(res) => res,
        Err(_err) => {
            error!("Error while getting user");
            return Ok("[]".to_owned());
        }
    };

    let user = serde_json::from_str::<IdentifiedUser>(&identified_user).unwrap();

    let res = match SecretManager::query_secrets(&mut Database { conn: connection }, &user.userid) {
        Ok(res) => res,
        Err(_err) => {
            error!("Error while getting secrets");
            return Ok("[]".to_owned());
        }
    };

    Ok(serde_json::to_string(&res).unwrap())
}

#[tauri::command]
pub fn create_secret(
    app_state: State<AppState>,
    token: String,
    name: String,
    content: String,
) -> Result<usize, String> {
    let pool = app_state
        .db_manager
        .lock()
        .unwrap()
        .clone()
        .expect("Error sending request");

    let connection: DBConnection = pool.get().unwrap();
    let identified_user = match me(app_state, token) {
        Ok(res) => res,
        Err(_err) => {
            error!("Error while getting user");
            _err
        }
    };

    let user = serde_json::from_str::<IdentifiedUser>(&identified_user).unwrap();

    let secret = match <dyn SecretManager>::create_secret(
        &mut Database { conn: connection },
        &name,
        &content,
        &user.userid,
    ) {
        Ok(res) => res,
        Err(_err) => {
            error!("Error creating secret");
            return Ok(0);
        }
    };

    Ok(secret)
}

#[tauri::command]
pub fn delete_secret(app_state: State<AppState>, id: i32) { /*
                                                            let connection = (app_state
                                                            .db
                                                            .lock()
                                                            .unwrap()
                                                            .as_ref()
                                                            .expect("")).as_deref();

                                                            match <dyn SecretManager>::delete_secret(&mut Database { connection }, &id) {
                                                                Ok(_res) => (),
                                                                Err(_err) => {
                                                                    error!("Error deleting secret");
                                                                }
                                                            }*/
}

#[tauri::command]
pub fn clean_server() {
    let mut connection = Database::new();
    Database::clean_server(&mut connection);
}
