use crate::schema::{card, deck};
use crate::utils::db;
use diesel::Connection;
use diesel::prelude::QueryDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Route, delete, get, routes as rocket_routes};

use crate::models::{Deck, DeckWithCards};

#[get("/")]
fn get_decks() -> String {
    let mut db_connection = db::get_connection().lock().unwrap();
    let decks: Vec<Deck> = deck::table
        .select(deck::all_columns)
        .load(&mut *db_connection)
        .unwrap();

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
        .load(&mut *db_connection)
    {
        cards
    } else {
        return Err(status::NotFound(format!(
            "Cards for deck with id {} could not be loaded",
            id
        )));
    };

    Ok(Json(DeckWithCards { deck, card_ids }))
}

#[delete("/<id>")]
fn delete_deck(id: i32) -> Result<status::NoContent, status::Custom<String>> {
    let mut db_connection = db::get_connection().lock().unwrap();

    rocket::info!("Deleting deck {id}");

    // Use a transaction to ensure atomicity
    db_connection
        .transaction::<_, diesel::result::Error, _>(|conn| {
            // First, delete all cards associated with this deck
            let num_cards =
                diesel::delete(card::table.filter(card::deck_id.eq(id))).execute(conn)?;

            rocket::info!("Deleted {} cards from deck {}", num_cards, id);

            // Then delete the deck itself
            let num_decks = diesel::delete(deck::table.filter(deck::id.eq(id))).execute(conn)?;

            if num_decks == 0 {
                return Err(diesel::result::Error::NotFound);
            }

            Ok(())
        })
        .map(|_| {
            rocket::info!(
                "Deleted deck {id}. Number of decks left: {}",
                deck::table
                    .count()
                    .get_result::<i64>(&mut *db_connection)
                    .unwrap()
            );
            status::NoContent
        })
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                status::Custom(Status::NotFound, format!("Deck with id {} not found", id))
            }
            _ => {
                eprintln!("Database error deleting deck {}: {:?}", id, e);
                status::Custom(
                    Status::InternalServerError,
                    "Failed to delete deck".to_string(),
                )
            }
        })
}

pub fn routes() -> Vec<Route> {
    rocket_routes![delete_deck, get_deck, get_decks]
}
