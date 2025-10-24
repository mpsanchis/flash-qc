use rocket::http::Status;
use rocket::{Route, get, routes as rocket_routes};

#[get("/health")]
fn health() -> Status {
    Status::Ok
}

pub fn routes() -> Vec<Route> {
    rocket_routes![health,]
}
