use rocket::launch;
extern crate rocket;

mod models;
mod schema;
mod system;
mod root;

#[launch]
fn rocket() -> _ {
    rocket::build()
      .attach(system::Cors)
      .mount("/", root::routes())
      .mount("/system", system::routes())
}