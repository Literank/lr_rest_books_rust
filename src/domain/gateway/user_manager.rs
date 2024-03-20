use std::error::Error;

use crate::domain::model;

pub trait UserManager: Send + Sync {
    fn create_user(&self, u: &model::User) -> Result<u32, Box<dyn Error>>;
    fn get_user_by_email(&self, email: &str) -> Result<Option<model::User>, Box<dyn Error>>;
}
