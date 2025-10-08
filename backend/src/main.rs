use rocket::{get, launch, routes};
extern crate rocket;

mod models;
mod schema;
mod system;


#[get("/")]
pub fn index() -> &'static str {
  "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
      .attach(system::Cors)
      .mount("/", routes![index])
      .mount("/system", system::routes())
}