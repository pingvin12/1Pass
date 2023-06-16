use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{Engine, engine};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use once_cell::sync::Lazy;

use std::sync::Mutex;
use getrandom::getrandom;
extern crate keyring;

static ENCRYPTION_KEY: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_owned()));

pub fn encrypt_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    password_hash.into()
}

pub fn verify_password(password: String, hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&hash).unwrap();

    let result = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    result.into()
}

pub fn encrypt_data(data: String) -> String {
    let key = ENCRYPTION_KEY.lock().unwrap().to_string();
    let mc = new_magic_crypt!(key, 256);

    let encrypted_string = mc.encrypt_str_to_base64(data);

    encrypted_string.into()
}

pub fn decrypt_data(data: String) -> String {
    let key = ENCRYPTION_KEY.lock().unwrap().to_string();
    let mc = new_magic_crypt!(key, 256);

    let decrypted_string = mc
        .decrypt_base64_to_string(data)
        .unwrap_or_else(|error| error.to_string().into());

    decrypted_string.into()
}

pub fn set_entry(name: String, data: String, service: String) -> String {
    let entry = keyring::Entry::new(&service, &name);

    let res = entry.unwrap().set_password(data.as_str());

    match res {
        Ok(_) => "ok".into(),
        Err(_) => "error".into(),
    }
}

pub fn get_entry(name: String, service: String) -> String {
    let entry = keyring::Entry::new(&service, &name);

    let item = entry
        .unwrap()
        .get_password()
        .unwrap_or_else(|_error| "error".into());

    item.into()
}

pub fn delete_entry(name: String, service: String) {
    let entry = keyring::Entry::new(&service, &name);

    let _item = entry.unwrap().delete_password();
}

pub fn receive_encryption_key(key: String) {
    *ENCRYPTION_KEY.lock().unwrap() = key
}

pub fn set_encryption_key(service: String) {
    *ENCRYPTION_KEY.lock().unwrap() = get_entry("encryptionKey".to_string(), service)
}

pub fn generate_encryption_key(length: usize) -> String {
    let mut key = vec![0u8; length];
    getrandom(&mut key).expect("Failed to generate encryption key");
    engine::general_purpose::STANDARD.encode(key)
}