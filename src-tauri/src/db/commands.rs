use std::error::Error;

use crate::db::database::Database;

use super::domain::auth::JwtToken::JwtToken;

#[tauri::command]
pub fn register(username: String, password: String, email: String) -> Result<(), ()> {
    let mut connection = Database::new();
    let res = Database::register(&mut connection, &username, &password, &email); // map err to string
    match res {
        Ok(result) => Ok(result),
        Err(_err) => todo!("error handling"),
    }
}

#[tauri::command]
pub fn auth(username: String, password: String) -> Result<String, ()> {
    let mut connection = Database::new();
    let res = Database::authenticate(&mut connection, &username, &password);
    match res {
        Ok(result) => Ok(result.token),
        Err(_err) => todo!("error handling"),
    }
}

/* !todo()
#[tauri::command]
pub fn secrets(JwtToken: JwtToken) -> Result<(), Box<dyn Error>>
{
    let mut connection = Database::new();
    let res = Database::secrets()
    Ok(())
}
*/
