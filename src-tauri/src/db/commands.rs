use crate::db::database::Database;

use super::encryption::{generate_encryption_key, self};

#[tauri::command]
pub fn register(username: String, password: String, email: String) -> Result<String, ()> {
    let mut connection = Database::new();
    let res = Database::register(&mut connection, &username, &password, &email); // map err to string
    match res {
        Ok(result) => Ok(result),
        Err(_err) => todo!("error handling"),
    }
}

#[tauri::command]
pub fn auth(email: String, password: String) -> Result<String, ()> {
    let mut connection = Database::new();
    let res = Database::authenticate(&mut connection, &email, &password);
    match res {
        Ok(result) => Ok(result.token),
        Err(_err) => todo!("error handling"),
    }
}

#[tauri::command]
pub fn me(token: String) -> Result<String, String> {
    let mut connection = Database::new();
    let res = Database::me(&mut connection, &token);
    match res {
        Ok(res) => Ok(res),
        Err(_err) => todo!()
    }
}

#[tauri::command]
pub fn logout() {

}

#[tauri::command]
pub fn init_hosting() {
    let generated = generate_encryption_key(32);
    encryption::set_encryption_key(generated);
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
