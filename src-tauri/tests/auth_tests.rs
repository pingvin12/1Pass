pub mod auth_tests {
    use app::db::commands;

    #[test]
    pub fn register_success() {
        let res = commands::register(
            "tdest@test.com".to_string(),
            "test".to_string(),
            "test@test.com".to_string(),
        );
    }

    #[test]
    pub fn login_success() {
        let token = commands::auth("tdest@test.com".to_string(), "test".to_string()).unwrap();
    }

    #[test]
    pub fn login_fail() {
        let token = commands::auth("tesssst@test.com".to_string(), "test".to_string());
    }

    #[test]
    pub fn get_me_success() {
        let token = commands::auth("tdest@test.com".to_string(), "test".to_string()).unwrap();
        let res = commands::me(token).unwrap();
        assert_ne!(res, "")
    }
}
