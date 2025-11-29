use diesel::prelude::QueryDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::response::status;
use rocket::serde::json::{Json, Value, json};
use rocket::{Route, get, routes as rocket_routes};

use crate::auth::BearerAuth;
use crate::models::Card;
use crate::schema::card;
use crate::utils::db;
use crate::utils::gen_error::GenericError;

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

// This endpoint is only a demonstration of how an endpoint would be protected under Bearer Auth
#[get("/under_auth")]
fn under_auth(user_data: BearerAuth) -> Result<Value, GenericError> {
    Ok(json!({"status": "success", "message": format!("Hello, {}!", user_data.user.username)}))
}

pub fn routes() -> Vec<Route> {
    rocket_routes![get_card, under_auth]
}
