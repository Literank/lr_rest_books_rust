#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/ping")]
fn ping() -> content::RawJson<&'static str> {
    content::RawJson("{\"message\": \"pong\"}")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ping])
}
