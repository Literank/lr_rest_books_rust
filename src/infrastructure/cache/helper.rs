use std::error::Error;

pub trait Helper: Send + Sync {
    fn save(&self, key: &str, value: &str) -> Result<(), Box<dyn Error>>;
    fn load(&self, key: &str) -> Result<Option<String>, Box<dyn Error>>;
}
