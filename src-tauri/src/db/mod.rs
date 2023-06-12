pub mod domain {
    pub mod auth {
        pub mod JwtToken;
        pub mod UserObject;
    }
    pub mod secret {
        pub mod SecretObject;
    }
}

pub mod handlers {
    pub mod news {
        pub mod getnews;
    }
    pub mod secret {
        pub mod getsecret;
        pub mod createsecret;
    }
}

pub mod commands;
pub mod database;
pub mod encryption;