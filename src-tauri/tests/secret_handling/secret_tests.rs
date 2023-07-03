pub mod auth_tests {
    use app::db::commands;

    use crate::clients;

    #[test]
    pub fn create_secret_success() {
        let client = clients::test_client::TestClient::new();
        
        let create_secret = commands::create_secret(client.get_token(), "test".to_owned(), "test_content".to_owned()).unwrap();
        
        assert_eq!(create_secret, 1);
        client.clean_client();
    }

    #[test]
    pub fn list_secrets_success() {
        let client = clients::test_client::TestClient::new();
        
        let _ = commands::create_secret(client.get_token(), "test".to_owned(), "test_content".to_owned()).unwrap();
        let query = commands::get_secrets(client.get_token()).unwrap();
        
        assert!(query.contains("title\":\"test\",\"content\":\"test_content\","));

        client.clean_client();
    }

}
