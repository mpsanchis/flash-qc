use rocket::launch;
extern crate rocket;
use std::sync::Arc;
use std::sync::Mutex;

use flashqc_backend::utils::token_store::TokenStore;
use flashqc_backend::{auth, card, deck, root, system};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Arc::new(Mutex::new(TokenStore::new())))
        .attach(system::Cors)
        .mount("/", root::routes())
        .mount("/auth", auth::routes())
        .mount("/system", system::routes())
        .mount("/decks", deck::routes())
        .mount("/cards", card::routes())
}
