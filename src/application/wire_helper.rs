use std::sync::Arc;

use crate::domain::gateway;
use crate::infrastructure::database;
use crate::infrastructure::Config;

pub struct WireHelper {
    persistence: Arc<database::MySQLPersistence>,
}

impl WireHelper {
    pub fn new(c: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let persistence = Arc::new(database::MySQLPersistence::new(&c.db.dsn)?);
        Ok(WireHelper { persistence })
    }

    pub fn book_manager(&self) -> Arc<dyn gateway::BookManager> {
        Arc::clone(&self.persistence) as Arc<dyn gateway::BookManager>
    }
}
