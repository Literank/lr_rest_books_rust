use std::error::Error;

use crate::domain::model;

pub trait ReviewManager: Send + Sync {
    fn create_review(&self, b: &model::Review) -> Result<String, Box<dyn Error>>;
    fn update_review(&self, id: &str, b: &model::Review) -> Result<(), Box<dyn Error>>;
    fn delete_review(&self, id: &str) -> Result<(), Box<dyn Error>>;
    fn get_review(&self, id: &str) -> Result<Option<model::Review>, Box<dyn Error>>;
    fn get_reviews_of_book(&self, book_id: u32) -> Result<Vec<model::Review>, Box<dyn Error>>;
}
