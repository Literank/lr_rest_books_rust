use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};

use crate::domain::model::UserPermission;
use crate::RestHandler;

// Define a struct to hold the permission level required for the route
pub struct PermCheck;

// Implement FromRequest trait to perform permission check
#[rocket::async_trait]
impl<'r> FromRequest<'r> for PermCheck {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_header = match request.headers().get_one("Authorization") {
            Some(header) => header,
            None => return request::Outcome::Error((Status::Unauthorized, "Token is required")),
        };
        let token = auth_header.trim_start_matches("Bearer ");
        let rest_handler = request.rocket().state::<RestHandler>().unwrap();
        // Check user permission against required permission
        match rest_handler
            .user_operator
            .has_permission(token, UserPermission::PermAuthor)
        {
            Ok(b) => {
                if b {
                    request::Outcome::Success(PermCheck {})
                } else {
                    request::Outcome::Error((Status::Unauthorized, "Unauthorized"))
                }
            }
            Err(_) => request::Outcome::Error((Status::BadRequest, "Invalid token")),
        }
    }
}
