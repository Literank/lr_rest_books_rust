#[macro_use]
extern crate rocket;

mod adapter;
mod application;
mod domain;
mod infrastructure;

use crate::adapter::router::*;
use crate::infrastructure::parse_config;

const CONFIG_FILE: &str = "config.toml";

#[launch]
fn rocket() -> _ {
    let c = parse_config(CONFIG_FILE);
    let wire_helper = application::WireHelper::new(&c).expect("Failed to create WireHelper");
    let r = adapter::make_router(&wire_helper);
    rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("port", c.app.port))
                .merge(("address", c.app.address)),
        )
        .manage(r)
        .mount(
            "/",
            routes![
                health_check,
                get_books,
                get_book,
                create_book,
                update_book,
                delete_book,
                get_reviews_of_book,
                get_review,
                create_review,
                update_review,
                delete_review,
                user_sign_up,
                user_sign_in
            ],
        )
}
