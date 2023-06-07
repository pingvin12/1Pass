pub mod auth_tests {
    use app::db::commands;

    #[test]
    pub fn list_secrets_success() {
        let token = commands::auth("test@test.com".to_string(), "test".to_string()).unwrap();
    }
}
