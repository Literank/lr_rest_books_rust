#[macro_use]
extern crate rocket;

mod adapter;
mod application;
mod domain;
mod infrastructure;
use crate::adapter::router::*;
use crate::infrastructure::{ApplicationConfig, Config, DBConfig};

#[launch]
fn rocket() -> _ {
    let c = Config {
        app: ApplicationConfig { port: 8000 },
        db: DBConfig {
            file_name: "test.db".to_string(),
            dsn: "mysql://test_user:test_pass@127.0.0.1:3306/lr_book".to_string(),
        },
    };
    let wire_helper = application::WireHelper::new(&c).expect("Failed to create WireHelper");
    let r = adapter::make_router(&wire_helper);
    rocket::build().manage(r).mount(
        "/",
        routes![
            health_check,
            get_books,
            get_book,
            create_book,
            update_book,
            delete_book
        ],
    )
}
