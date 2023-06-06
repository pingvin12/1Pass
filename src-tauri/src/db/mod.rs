pub mod domain {
    pub mod auth {
        pub mod JwtToken;
        pub mod UserObject;
    }
    pub mod secret {
        pub mod SecretObject;
    }
}
pub mod commands;
pub mod database;
pub mod encryption;
