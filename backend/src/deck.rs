use crate::schema::{card, deck};
use crate::utils::db;
use diesel::prelude::QueryDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::response::status;
use rocket::serde::{json::Json};
use rocket::{Route, get, routes as rocket_routes};

use crate::models::{Deck, DeckWithCards};

#[get("/")]
fn get_decks() -> String {
    let mut db_connection = db::get_connection().lock().unwrap();

    let decks: Vec<Deck> = deck::table
        .select((deck::id, deck::name))
        .load(&mut *db_connection)
        .unwrap()
        .into_iter()
        .map(|(id, name)| Deck { id, name })
        .collect();

    serde_json::to_string(&decks).unwrap()
}


#[get("/<id>")]
fn get_deck(id: i32) -> Result<Json<DeckWithCards>, status::NotFound<String>> {
    let mut db_connection = db::get_connection().lock().unwrap();

    let deck: Deck = if let Ok(deck) = deck::table
        .select(deck::all_columns)
        .filter(deck::id.eq(id))
        .first(&mut *db_connection)
    {
        deck
    } else {
        return Err(status::NotFound(format!("Deck with id {} not found", id)));
    };

    let card_ids: Vec<i32> = if let Ok(cards) = card::table
        .filter(card::deck_id.eq(id))
        .order_by(card::id.asc())
        .select(card::id)
        .load(&mut *db_connection) {
            cards
        } else {
            return Err(status::NotFound(format!("Cards for deck with id {} could not be loaded", id)));
        };

    Ok(Json(DeckWithCards{deck, card_ids}))
}

pub fn routes() -> Vec<Route> {
    rocket_routes![get_deck, get_decks]
}
