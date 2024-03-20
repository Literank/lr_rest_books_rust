use std::sync::Arc;

use crate::domain::gateway;
use crate::infrastructure::cache;
use crate::infrastructure::database;
use crate::infrastructure::token;
use crate::infrastructure::Config;

pub struct WireHelper {
    sql_persistence: Arc<database::MySQLPersistence>,
    no_sql_persistence: Arc<database::MongoPersistence>,
    kv_store: Arc<cache::RedisCache>,
    token_keeper: Arc<token::Keeper>,
}

impl WireHelper {
    pub fn new(c: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let sql_persistence =
            Arc::new(database::MySQLPersistence::new(&c.db.dsn, c.app.page_size)?);
        let no_sql_persistence = Arc::new(database::MongoPersistence::new(
            &c.db.mongo_uri,
            &c.db.mongo_db_name,
        )?);
        let kv_store = Arc::new(cache::RedisCache::new(&c.cache.redis_uri)?);
        let token_keeper = Arc::new(token::Keeper::new(
            c.app.token_secret.clone(),
            c.app.token_hours,
        ));
        Ok(WireHelper {
            sql_persistence,
            no_sql_persistence,
            kv_store,
            token_keeper,
        })
    }

    pub fn book_manager(&self) -> Arc<dyn gateway::BookManager> {
        Arc::clone(&self.sql_persistence) as Arc<dyn gateway::BookManager>
    }

    pub fn user_manager(&self) -> Arc<dyn gateway::UserManager> {
        Arc::clone(&self.sql_persistence) as Arc<dyn gateway::UserManager>
    }

    pub fn perm_manager(&self) -> Arc<dyn gateway::PermissionManager> {
        Arc::clone(&self.token_keeper) as Arc<dyn gateway::PermissionManager>
    }

    pub fn review_manager(&self) -> Arc<dyn gateway::ReviewManager> {
        Arc::clone(&self.no_sql_persistence) as Arc<dyn gateway::ReviewManager>
    }

    pub fn cache_helper(&self) -> Arc<dyn cache::Helper> {
        Arc::clone(&self.kv_store) as Arc<dyn cache::Helper>
    }
}
