use dotenv::dotenv;
use std::env;
use postgrest::Postgrest;
use super::encryption::{self, encrypt_password, verify_password};

struct Database {
    client: Postgrest,
    jwttoken: String,
}


impl Database {
    fn new() -> Self {
        dotenv().ok();

        let api_url = env::var("ENDPOINT").unwrap();
        let api_key = env::var("API_KEY").unwrap();
        let jwttoken = env::var("JWT_TOKEN").unwrap();
        let client = Postgrest::new(api_url.as_str())
            .insert_header("apikey", api_key);

        Database { client, jwttoken }
    }

    async fn authenticate(&self, username: String, password: String) -> Result<bool, Box<dyn std::error::Error>> {
        let response = self.client
            .from("users")
            .auth(self.jwttoken.as_str())
            .eq("username", username)
            .select("password")
            .execute().await?;
        Ok(verify_password(password, response.text().await?))
    }

    async fn register(&self, username: String, password: String) -> Result<bool, Box<dyn std::error::Error>> {
        let password_modified = encrypt_password(password);
        let response = self.client
            .from("users")
            .auth(self.jwttoken.as_str())
            .insert(format!("[username: {},password: {}]", username.as_str(), password_modified.as_str()))
            .execute().await?;

        Ok(response.status().is_success())
    }
}

#[tauri::command]
pub async fn auth_user(username: String, password: String) -> bool
{
    let connect = Database::new();
    let res = connect.authenticate(username, password);
    res.await.unwrap()
}

#[tauri::command]
pub async fn register_user(username: String, password: String) -> bool
{
    let connect = Database::new();
    let res = connect.register(username, password);
    res.await.unwrap()
}