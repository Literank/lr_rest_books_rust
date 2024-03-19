use std::error::Error;

use crate::domain::model;

pub trait BookManager: Send + Sync {
    fn create_book(&self, b: &model::Book) -> Result<u32, Box<dyn Error>>;
    fn update_book(&self, id: u32, b: &model::Book) -> Result<(), Box<dyn Error>>;
    fn delete_book(&self, id: u32) -> Result<(), Box<dyn Error>>;
    fn get_book(&self, id: u32) -> Result<Option<model::Book>, Box<dyn Error>>;
    fn get_books(&self, offset: u32) -> Result<Vec<model::Book>, Box<dyn Error>>;
}
