use rocket::launch;
extern crate rocket;

mod models;
mod root;
mod schema;
mod system;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(system::Cors)
        .mount("/", root::routes())
        .mount("/system", system::routes())
}
