use std::sync::Arc;

use crate::domain::gateway;
use crate::domain::model;
use crate::infrastructure::cache;

const BOOKS_KEY: &str = "lr-books";

pub struct BookOperator {
    book_manager: Arc<dyn gateway::BookManager>,
    cache_helper: Arc<dyn cache::Helper>,
}

impl BookOperator {
    pub fn new(b: Arc<dyn gateway::BookManager>, c: Arc<dyn cache::Helper>) -> Self {
        BookOperator {
            book_manager: b,
            cache_helper: c,
        }
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
        let raw_value = self.cache_helper.load(BOOKS_KEY)?;
        if let Some(v) = raw_value {
            let cached_books = serde_json::from_str(&v)?;
            Ok(cached_books)
        } else {
            let fetched_books = self.book_manager.get_books()?;
            let v = serde_json::to_string(&fetched_books)?;
            self.cache_helper.save(BOOKS_KEY, &v)?;
            Ok(fetched_books)
        }
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
