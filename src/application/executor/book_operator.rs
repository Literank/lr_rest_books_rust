use std::sync::Arc;

use crate::domain::gateway;
use crate::domain::model;

pub struct BookOperator {
    book_manager: Arc<dyn gateway::BookManager>,
}

impl BookOperator {
    pub fn new(b: Arc<dyn gateway::BookManager>) -> Self {
        BookOperator { book_manager: b }
    }

    pub fn create_book(&self, b: model::Book) -> Result<model::Book, Box<dyn std::error::Error>> {
        let id = self.book_manager.create_book(&b)?;
        let mut book = b;
        book.id = id;
        Ok(book)
    }

    pub fn get_book(&self, id: u32) -> Result<Option<model::Book>, Box<dyn std::error::Error>> {
        self.book_manager.get_book(id)
    }

    pub fn get_books(&self) -> Result<Vec<model::Book>, Box<dyn std::error::Error>> {
        self.book_manager.get_books()
    }

    pub fn update_book(
        &self,
        id: u32,
        b: model::Book,
    ) -> Result<model::Book, Box<dyn std::error::Error>> {
        self.book_manager.update_book(id, &b)?;
        Ok(b)
    }

    pub fn delete_book(&self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.book_manager.delete_book(id)
    }
}
