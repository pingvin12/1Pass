pub mod encryption_tests {
    use onepass::db::encryption;

    #[test]
    pub fn encrypt_data_success() {
        let data = encryption::encrypt_data("data".to_string());
        assert_eq!(data.to_string(), "2Tr/he/Nmf5Ix4woroGM+g==".to_string());
    }

    #[test]
    pub fn encrypt_data_fail() {
        let data = encryption::encrypt_data("data".to_string());
        assert_ne!(data.to_string(), "wrong".to_string());
    }

    #[test]
    pub fn decrypt_data_success() {
        let data = encryption::decrypt_data("2Tr/he/Nmf5Ix4woroGM+g==".to_string());
        assert_eq!(data.to_string(), "data".to_string());
    }

    #[test]
    pub fn decrypt_data_fail() {
        let data = encryption::decrypt_data("M5JfgcQvCRVm+0Gfj6dIAA==".to_string());
        assert_ne!(data.to_string(), "wrong".to_string());
    }
}
