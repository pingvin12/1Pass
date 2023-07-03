pub mod auth_tests {
    use app::db::commands;
    use crate::clients;

    #[test]
    pub fn get_me_success() {
        let client = clients::test_client::TestClient::new();
        let res = commands::me(client.get_token()).unwrap();
        assert!(res.contains("{\"username\":\"tdest@test.com\",\"email\":\"test@test.com\",\"userid\":1,"));
        client.clean_client();
    }
}
