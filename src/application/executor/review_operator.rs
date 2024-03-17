use std::sync::Arc;

use chrono::Utc;

use crate::application::dto;
use crate::domain::gateway;
use crate::domain::model;

pub struct ReviewOperator {
    review_manager: Arc<dyn gateway::ReviewManager>,
}

impl ReviewOperator {
    pub fn new(b: Arc<dyn gateway::ReviewManager>) -> Self {
        ReviewOperator { review_manager: b }
    }

    pub fn create_review(
        &self,
        body: &dto::ReviewBody,
    ) -> Result<model::Review, Box<dyn std::error::Error>> {
        let now = Utc::now();
        let review = model::Review {
            id: String::new(),
            book_id: body.book_id,
            author: body.author.clone(),
            title: body.title.clone(),
            content: body.content.clone(),
            created_at: now,
            updated_at: now,
        };
        let id = self.review_manager.create_review(&review)?;
        Ok(model::Review { id, ..review })
    }

    pub fn get_review(
        &self,
        id: &str,
    ) -> Result<Option<model::Review>, Box<dyn std::error::Error>> {
        self.review_manager.get_review(id)
    }

    pub fn get_reviews_of_book(
        &self,
        book_id: u32,
    ) -> Result<Vec<model::Review>, Box<dyn std::error::Error>> {
        self.review_manager.get_reviews_of_book(book_id)
    }

    pub fn update_review(
        &self,
        id: &str,
        body: dto::ReviewBody,
    ) -> Result<model::Review, Box<dyn std::error::Error>> {
        if body.title.is_empty() || body.content.is_empty() {
            return Err("Required field cannot be empty".into());
        }
        let now = Utc::now();
        let review = model::Review {
            id: id.to_string(),
            book_id: body.book_id,
            author: body.author.clone(),
            title: body.title.clone(),
            content: body.content.clone(),
            created_at: now,
            updated_at: now,
        };
        self.review_manager.update_review(id, &review)?;
        Ok(review)
    }

    pub fn delete_review(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.review_manager.delete_review(id)
    }
}
