use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::sync::Mutex;

static ENCRYPTION_KEY: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_owned()));

#[tauri::command]
pub fn encrypt_pass(pass: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let pass_hash = argon2.hash_password(pass.as_bytes(), &salt).unwrap().to_string();
    pass_hash.into()
}

#[tauri::command]
pub fn encrypt_data(data: String) -> String {
    let key = ENCRYPTION_KEY.lock().unwrap().to_string();
    let mc = new_magic_crypt!(key, 256);
    let encrypted_data = mc.encrypt_str_to_base64(data);
    encrypted_data.into()
}

#[tauri::command]
pub fn decrypt_data(data: String) -> String {
    let key = ENCRYPTION_KEY.lock().unwrap().to_string();
    let mc = new_magic_crypt!(key, 256);
    let decrypted_string = mc
        .decrypt_base64_to_string(data)
        .unwrap_or_else(|error| "error".into());
    decrypted_string.into()
}


#[tauri::command]
pub fn set_entry(name: String, data: String, service: String) -> String {
    let entry = keyring::Entry::new(&service, &name);

    let res = entry.set_password(data.as_str());

    match res {
        Ok(_) => "ok".into(),
        Err(_) => "error".into(),
    }
}

#[tauri::command]
pub fn get_entry(name: String, service: String) -> String {
    let entry = keyring::Entry::new(&service, &name);

    let item = entry.get_password().unwrap_or_else(|error| "error".into());

    item.into()
}

#[tauri::command]
pub fn delete_entry(name: String, service: String) {
    let entry = keyring::Entry::new(&service, &name);

    let item = entry.delete_password();
}

#[tauri::command]
pub fn receive_encryption_key(key: String) {
    *ENCRYPTION_KEY.lock().unwrap() = key
}

#[tauri::command]
pub fn set_encryption_key(service: String) {
    *ENCRYPTION_KEY.lock().unwrap() = get_entry("encryptionKey".to_string(), service)
}