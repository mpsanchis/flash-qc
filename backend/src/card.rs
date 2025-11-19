use diesel::prelude::QueryDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Route, get, routes as rocket_routes};

use crate::models::Card;
use crate::schema::card;
use crate::utils::db;

#[get("/<id>")]
fn get_card(id: i32) -> Result<Json<Card>, status::NotFound<String>> {
    let mut db_connection = db::get_connection().lock().unwrap();

    let card: Card = if let Ok(card) = card::table
        .select(card::all_columns)
        .filter(card::id.eq(id))
        .first(&mut *db_connection)
    {
        card
    } else {
        return Err(status::NotFound(format!("Card with id {id} not found")));
    };

    Ok(Json(card))
}

pub fn routes() -> Vec<Route> {
    rocket_routes![get_card,]
}
