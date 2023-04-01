use dotenv::dotenv;
use std::env;
use postgrest::Postgrest;

struct Database {
    client: Postgrest,
}

impl Database {
    fn new() -> Self {
        dotenv().ok();

        let api_url = env::var("ENDPOINT").unwrap();
        let api_key = env::var("API_KEY").unwrap();
        let client = Postgrest::new(api_url.as_str())
            .insert_header("apikey", api_key);

        Database { client }
    }

    async fn authenticate(&self, username: &str, password: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let response = self.client
            .from("users")
            .eq("username", username)
            .eq("password", password)
            .select("id")
            .execute().await?;

        Ok(response.status().is_success())
    }

    async fn register(&self, username: &str, password: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let response = self.client
            .from("users")
            .insert(r#"[
                "username": username,
                "password": password,
            ]"#)
            .execute().await?;

        Ok(response.status().is_success())
    }
}