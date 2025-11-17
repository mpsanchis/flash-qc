use rocket::launch;
extern crate rocket;

use flashqc_backend::{deck, root, system};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(system::Cors)
        .mount("/", root::routes())
        .mount("/system", system::routes())
        .mount("/decks", deck::routes())
}
