pub mod auth_tests {
    use app::db::database;


    #[test]
    pub fn list_secrets_success() {
        let token = database::login("test@test.com".to_string(), "test".to_string()).unwrap();
        assert_eq!(token.validtill, 86400);
    }
}