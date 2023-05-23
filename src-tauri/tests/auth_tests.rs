pub mod auth_tests {
    use app::db::database;

    #[test]
    pub fn register_success() {
        let res = database::register("tdest@test.com".to_string(), "test".to_string(),"test@test.com".to_string());
        assert_eq!(true, res.unwrap());
    }

    #[test]
    pub fn login_success() {
        let token = database::login("test@test.com".to_string(), "test".to_string()).unwrap();
        assert_eq!(token.validtill, 86400);
    }

    #[test]
    pub fn login_fail() {
        let token = database::login("tesssst@test.com".to_string(), "test".to_string());
    }

}