pub mod domain {
    pub mod auth {
        pub mod jwt_token;
        pub mod user_object;
    }
    pub mod secret {
        pub mod secret_object;
    }
    pub mod news {
        pub mod news_object;
    }
}

pub mod handlers {
    pub mod news {
        pub mod news_handler;
    }
    pub mod secret {
        pub mod secret_handler;
    }
}

pub mod commands;
pub mod database;
pub mod encryption;
