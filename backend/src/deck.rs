use crate::schema::{deck, plugincard};
use crate::utils::db;
use diesel::prelude::QueryDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::response::status;
use rocket::serde::{Serialize, json::Json};
use rocket::{Route, get, routes as rocket_routes};

#[derive(Serialize)]
struct DeckProxy {
    id: i32,
    name: String,
}

impl DeckProxy {
    fn new(id: i32, name: String) -> Self {
        DeckProxy { id, name }
    }
}

#[get("/")]
fn get_decks() -> String {
    let mut db_connection = db::get_connection().lock().unwrap();

    let decks: Vec<DeckProxy> = deck::table
        .select((deck::id, deck::name))
        .load(&mut *db_connection)
        .unwrap()
        .into_iter()
        .map(|(id, name)| DeckProxy::new(id, name))
        .collect();

    serde_json::to_string(&decks).unwrap()
}

#[derive(Serialize)]
struct DeckInfo {
    id: i32,
    name: String,
    cards: Vec<PluginCardProxy>,
}

impl DeckInfo {
    fn new(id: i32, name: String, cards: Vec<PluginCardProxy>) -> Self {
        DeckInfo { id, name, cards }
    }
}

#[derive(Serialize)]
struct PluginCardProxy {
    id: i32,
    name: String,
}

#[get("/<id>")]
fn get_deck(id: i32) -> Result<Json<DeckInfo>, status::NotFound<String>> {
    let mut db_connection = db::get_connection().lock().unwrap();

    let name = if let Ok(name) = deck::table
        .select(deck::name)
        .filter(deck::id.eq(id))
        .first(&mut *db_connection)
    {
        name
    } else {
        return Err(status::NotFound(format!("Deck with id {} not found", id)));
    };

    let plugincards: Vec<PluginCardProxy> = plugincard::table
        .filter(plugincard::deck_id.eq(id))
        .order_by(plugincard::id.asc())
        .select((plugincard::id, plugincard::name))
        .load(&mut *db_connection)
        .unwrap()
        .into_iter()
        .map(|(id, name)| PluginCardProxy { id, name })
        .collect();

    Ok(Json(DeckInfo::new(id, name, plugincards)))
}

pub fn routes() -> Vec<Route> {
    rocket_routes![get_deck, get_decks,]
}
