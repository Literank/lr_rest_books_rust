use std::error::Error;
use std::sync::RwLock;

use redis::{Client, Commands, Connection};

use crate::infrastructure::cache::Helper;

const DEFAULT_TTL: u64 = 3600; // seconds

pub struct RedisCache {
    conn: RwLock<Connection>,
}

impl RedisCache {
    pub fn new(redis_uri: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::open(redis_uri)?;
        let conn = client.get_connection()?;
        Ok(Self {
            conn: RwLock::new(conn),
        })
    }
}

impl Helper for RedisCache {
    fn save(&self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.conn.write().unwrap();
        conn.set_ex(key, value, DEFAULT_TTL)?;
        Ok(())
    }

    fn load(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        // Caution: `conn.read()` doesn't work here
        let mut conn = self.conn.write().unwrap();
        let result: Option<String> = conn.get(key)?;
        Ok(result)
    }
}
