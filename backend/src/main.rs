use rocket::launch;
extern crate rocket;

use flashqc_backend::{auth, card, deck, root, system};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(system::Cors)
        .mount("/", root::routes())
        .mount("/auth", auth::routes())
        .mount("/system", system::routes())
        .mount("/decks", deck::routes())
        .mount("/cards", card::routes())
}
