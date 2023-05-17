pub mod auth_tests {
    use app::db::database;
    use futures;

    #[test]
    pub fn register_success() {
        let fut_res = database::register_user("test@test.com".to_string(), "test".to_string());
        let res = format!("{}", futures::executor::block_on(fut_res));
        
        assert_eq!(res.to_string(), true.to_string());
    }

    pub fn auth_success() {
      let fut_res = database::auth_user("test@test.com".to_string(), "test".to_string());
      let res = format!("{}", futures::executor::block_on(fut_res));
      
      assert_eq!(res.to_string(), true.to_string());
    }
}