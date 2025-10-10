use rocket::{get, routes as rocket_routes, Route};
use rocket::response::content::RawHtml;

#[get("/")]
pub fn index() -> &'static str {
  "Hello, world!"
}

#[get("/main-frame")]
fn main_frame() -> RawHtml<&'static str> {
  RawHtml(include_str!("./templates/under-construction.html"))
}

pub fn routes() -> Vec<Route> {
  rocket_routes![
    index,
    main_frame,
  ]
}
