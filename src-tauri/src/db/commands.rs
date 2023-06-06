use crate::db::database;

use super::domain::auth::JwtToken::JwtToken;


#[tauri::command]
pub fn register(username: String, password: String, email: String) -> Result<bool, String>
{
    let res = database::register(username, password, email); // map err to string
    match res {
        Ok(result) => Ok(result),
        Err(_err) => Err(_err),
    }
}

#[tauri::command]
pub fn auth(email: String, password: String) -> Result<String, String>
{
    let res = database::login(email, password).unwrap();
    Ok(res.token)
}

#[tauri::command]
pub fn secrets() -> Result<String, String>
{
    let res = database::return_secrets(JwtToken { token: "sss".into() });
    Ok(res.unwrap())
}