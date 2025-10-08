use rocket::http::Status;
use rocket::{get, routes as rocket_routes, Route};

#[get("/health")]
fn health() -> Status {
  Status::Ok
}

pub fn routes() -> Vec<Route> {
  rocket_routes![
    health,
  ]
}