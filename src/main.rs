#[macro_use]
extern crate rocket;

use rocket::response::content;

// Define a health endpoint handler, use `/health` or `/`
#[get("/")]
fn ping() -> content::RawJson<&'static str> {
    // Return a simple response indicating the server is healthy
    content::RawJson("{\"status\":\"ok\"}")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ping])
}
