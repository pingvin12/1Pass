use std::io;
use std::error::Error;
use app::db::database;
use diesel::PgConnection;

pub struct Testclient {
    pub database: database::Database,
    pub connection: Option<PgConnection>,
}

pub trait Client {
    fn init_context(&mut self) -> Result<(), Box<dyn Error>>;
    fn init(&self) -> Result<(), Box<dyn Error>>;
}

impl Client for Testclient {
    fn init_context(&mut self) -> Result<(), Box<dyn Error>> {
        
        Ok(())
    }

    fn init(&self) -> Result<(), Box<dyn Error>> {
        // Use the connection if it's available
        if let Some(connection) = &self.connection {
            // Perform some operation with the connection
            // ...
            Ok(())
        } else {
            // Connection not initialized
            Err("Connection not initialized".into())
        }
    }
}
