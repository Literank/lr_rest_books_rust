use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;

use crate::application;
use crate::application::dto;
use crate::application::executor;
use crate::domain::model;

pub struct RestHandler {
    book_operator: executor::BookOperator,
    review_operator: executor::ReviewOperator,
    user_operator: executor::UserOperator,
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

#[get("/books?<o>&<q>")]
pub fn get_books(
    rest_handler: &rocket::State<RestHandler>,
    o: Option<u32>,
    q: Option<&str>,
) -> Result<Json<Vec<model::Book>>, status::Custom<Json<ErrorResponse>>> {
    match rest_handler
        .book_operator
        .get_books(o.unwrap_or(0), q.unwrap_or(""))
    {
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

#[get("/books/<id>/reviews?<q>")]
pub fn get_reviews_of_book(
    rest_handler: &rocket::State<RestHandler>,
    id: u32,
    q: Option<&str>,
) -> Result<Json<Vec<model::Review>>, status::Custom<Json<ErrorResponse>>> {
    match rest_handler
        .review_operator
        .get_reviews_of_book(id, q.unwrap_or(""))
    {
        Ok(reviews) => Ok(Json(reviews)),
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[get("/reviews/<id>")]
pub fn get_review(
    rest_handler: &rocket::State<RestHandler>,
    id: &str,
) -> Result<Json<model::Review>, status::Custom<Json<ErrorResponse>>> {
    match rest_handler.review_operator.get_review(id) {
        Ok(review) => match review {
            Some(r) => Ok(Json(r)),
            None => Err(status::Custom(
                Status::NotFound,
                Json(ErrorResponse {
                    error: format!("review {id} not found"),
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

#[post("/reviews", format = "json", data = "<review>")]
pub fn create_review(
    rest_handler: &rocket::State<RestHandler>,
    review: Json<dto::ReviewBody>,
) -> Result<Json<model::Review>, status::Custom<Json<ErrorResponse>>> {
    match rest_handler
        .review_operator
        .create_review(&review.into_inner())
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

#[put("/reviews/<id>", format = "json", data = "<review>")]
pub fn update_review(
    rest_handler: &rocket::State<RestHandler>,
    id: &str,
    review: Json<dto::ReviewBody>,
) -> Result<Json<model::Review>, status::Custom<Json<ErrorResponse>>> {
    match rest_handler
        .review_operator
        .update_review(id, review.into_inner())
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

#[delete("/reviews/<id>")]
pub fn delete_review(
    rest_handler: &rocket::State<RestHandler>,
    id: &str,
) -> Result<status::NoContent, status::Custom<Json<ErrorResponse>>> {
    match rest_handler.review_operator.delete_review(id) {
        Ok(_) => Ok(status::NoContent),
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[post("/users", format = "json", data = "<uc>")]
pub fn user_sign_up(
    rest_handler: &rocket::State<RestHandler>,
    uc: Json<dto::UserCredential>,
) -> Result<Json<dto::User>, status::Custom<Json<ErrorResponse>>> {
    match rest_handler.user_operator.create_user(&uc.into_inner()) {
        Ok(u) => Ok(Json(u)),
        Err(err) => Err(status::Custom(
            Status::InternalServerError,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),
    }
}

#[post("/users/sign-in", format = "json", data = "<uc>")]
pub fn user_sign_in(
    rest_handler: &rocket::State<RestHandler>,
    uc: Json<dto::UserCredential>,
) -> Result<Json<dto::User>, status::Custom<Json<ErrorResponse>>> {
    match rest_handler.user_operator.sign_in(&uc.email, &uc.password) {
        Ok(u) => Ok(Json(u)),
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
        book_operator: executor::BookOperator::new(
            wire_helper.book_manager(),
            wire_helper.cache_helper(),
        ),
        review_operator: executor::ReviewOperator::new(wire_helper.review_manager()),
        user_operator: executor::UserOperator::new(wire_helper.user_manager()),
    }
}
