use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;

use crate::application;
use crate::application::executor;
use crate::domain::model;

pub struct RestHandler {
    book_operator: executor::BookOperator,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    error: String,
}

// Define a health endpoint handler, use `/health` or `/`
#[get("/")]
pub fn health_check() -> content::RawJson<&'static str> {
    // Return a simple response indicating the server is healthy
    content::RawJson("{\"status\":\"ok\"}")
}

#[get("/books")]
pub fn get_books(
    rest_handler: &rocket::State<RestHandler>,
) -> Result<Json<Vec<model::Book>>, status::Custom<Json<ErrorResponse>>> {
    match rest_handler.book_operator.get_books() {
        Ok(books) => Ok(Json(books)),
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[get("/books/<id>")]
pub fn get_book(
    rest_handler: &rocket::State<RestHandler>,
    id: u32,
) -> Result<Json<model::Book>, status::Custom<Json<ErrorResponse>>> {
    match rest_handler.book_operator.get_book(id) {
        Ok(book) => match book {
            Some(b) => Ok(Json(b)),
            None => Err(status::Custom(
                Status::NotFound,
                Json(ErrorResponse {
                    error: format!("book {id} not found"),
                }),
            )),
        },
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[post("/books", format = "json", data = "<book>")]
pub fn create_book(
    rest_handler: &rocket::State<RestHandler>,
    book: Json<model::Book>,
) -> Result<Json<model::Book>, status::Custom<Json<ErrorResponse>>> {
    match rest_handler.book_operator.create_book(book.into_inner()) {
        Ok(b) => Ok(Json(b)),
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[put("/books/<id>", format = "json", data = "<book>")]
pub fn update_book(
    rest_handler: &rocket::State<RestHandler>,
    id: u32,
    book: Json<model::Book>,
) -> Result<Json<model::Book>, status::Custom<Json<ErrorResponse>>> {
    match rest_handler
        .book_operator
        .update_book(id, book.into_inner())
    {
        Ok(b) => Ok(Json(b)),
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[delete("/books/<id>")]
pub fn delete_book(
    rest_handler: &rocket::State<RestHandler>,
    id: u32,
) -> Result<status::NoContent, status::Custom<Json<ErrorResponse>>> {
    match rest_handler.book_operator.delete_book(id) {
        Ok(_) => Ok(status::NoContent),
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

pub fn make_router(wire_helper: &application::WireHelper) -> RestHandler {
    RestHandler {
        book_operator: executor::BookOperator::new(wire_helper.book_manager()),
    }
}
