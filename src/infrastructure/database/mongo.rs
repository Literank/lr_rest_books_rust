use std::error::Error;

use mongodb::{
    bson::{doc, oid::ObjectId, DateTime, Regex},
    error::Error as MongoError,
    sync::{Client, Collection},
};

use crate::domain::gateway::ReviewManager;
use crate::domain::model::Review;

const COLL_REVIEW: &str = "reviews";
const ID_FIELD: &str = "_id";

pub struct MongoPersistence {
    coll: Collection<Review>,
}

impl MongoPersistence {
    pub fn new(mongo_uri: &str, db_name: &str) -> Result<Self, MongoError> {
        let client = Client::with_uri_str(mongo_uri)?;
        let coll = client.database(db_name).collection::<Review>(COLL_REVIEW);
        Ok(Self { coll })
    }
}

impl ReviewManager for MongoPersistence {
    fn create_review(&self, review: &Review) -> Result<String, Box<dyn Error>> {
        let result = self.coll.insert_one(review.clone(), None)?;
        let inserted_id = result
            .inserted_id
            .as_object_id()
            .expect("Failed to extract inserted ID");
        Ok(inserted_id.to_hex())
    }

    fn update_review(&self, id: &str, review: &Review) -> Result<(), Box<dyn Error>> {
        let object_id = ObjectId::parse_str(id)?;
        let update_values = doc! {
            "title": &review.title,
            "content": &review.content,
            "updated_at": DateTime::now(),
        };
        let filter = doc! { ID_FIELD: object_id };
        let _result = self
            .coll
            .update_one(filter, doc! { "$set": update_values }, None)?;
        Ok(())
    }

    fn delete_review(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let object_id = ObjectId::parse_str(id)?;
        self.coll.delete_one(doc! { ID_FIELD: object_id }, None)?;
        Ok(())
    }

    fn get_review(&self, id: &str) -> Result<Option<Review>, Box<dyn Error>> {
        let object_id = ObjectId::parse_str(id)?;
        let filter = doc! { ID_FIELD: object_id };
        let review = self.coll.find_one(filter, None)?;
        if let Some(review_doc) = review {
            Ok(Some(Review {
                id: id.to_string(),
                ..review_doc
            }))
        } else {
            Ok(None)
        }
    }

    fn get_reviews_of_book(
        &self,
        book_id: u32,
        keyword: &str,
    ) -> Result<Vec<Review>, Box<dyn Error>> {
        let mut filter = doc! { "book_id": book_id };
        if !keyword.is_empty() {
            filter = doc! {
                "$and": [
                    {
                        "$or": [
                            {"title": Regex{pattern: keyword.to_string(), options: String::from("i")}},
                            {"content": Regex{pattern: keyword.to_string(), options: String::from("i")}},
                        ]
                    },
                    {"book_id": book_id}
                ]
            }
        }
        let cursor = self.coll.find(filter, None)?;
        let mut reviews = Vec::new();
        for result in cursor {
            reviews.push(result?);
        }
        Ok(reviews)
    }
}
