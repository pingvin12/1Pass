pub mod auth_tests {
    use app::db::database;
    use futures;

    #[test]
    pub fn register_success() {
        let fut_res = database::register_user("test@test.com".to_string(), "test".to_string(),"test@test.com".to_string());
    }
}