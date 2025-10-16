mod models;
mod routes;
mod schema;

use rocket::{launch, routes};
use routes::index;

use crate::routes::Cors;

extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Cors).mount("/", routes![index])
}
