use std::sync::Mutex;

use mockall::automock;

use crate::db::database::DBPool;

pub struct AppState {
    pub db_manager: std::sync::Mutex<Option<DBPool>>,
}

#[automock]
pub trait AppStateTrait {
    fn new() -> Self;
    fn get_db_manager(&self) -> &Mutex<Option<DBPool>>;
}

impl AppStateTrait for AppState {
    fn new() -> Self {
        AppState {
            db_manager: Mutex::new(None),
        }
    }

    fn get_db_manager(&self) -> &Mutex<Option<DBPool>> {
        &self.db_manager
    }
}
