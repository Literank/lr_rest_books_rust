use std::sync::Arc;

use crate::domain::gateway;
use crate::infrastructure::cache;
use crate::infrastructure::database;
use crate::infrastructure::Config;

pub struct WireHelper {
    sql_persistence: Arc<database::MySQLPersistence>,
    no_sql_persistence: Arc<database::MongoPersistence>,
    kv_store: Arc<cache::RedisCache>,
}

impl WireHelper {
    pub fn new(c: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let sql_persistence = Arc::new(database::MySQLPersistence::new(&c.db.dsn)?);
        let no_sql_persistence = Arc::new(database::MongoPersistence::new(
            &c.db.mongo_uri,
            &c.db.mongo_db_name,
        )?);
        let kv_store = Arc::new(cache::RedisCache::new(&c.cache.redis_uri)?);
        Ok(WireHelper {
            sql_persistence,
            no_sql_persistence,
            kv_store,
        })
    }

    pub fn book_manager(&self) -> Arc<dyn gateway::BookManager> {
        Arc::clone(&self.sql_persistence) as Arc<dyn gateway::BookManager>
    }

    pub fn review_manager(&self) -> Arc<dyn gateway::ReviewManager> {
        Arc::clone(&self.no_sql_persistence) as Arc<dyn gateway::ReviewManager>
    }

    pub fn cache_helper(&self) -> Arc<dyn cache::Helper> {
        Arc::clone(&self.kv_store) as Arc<dyn cache::Helper>
    }
}
