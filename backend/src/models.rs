use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable, Deserialize, Selectable, Serialize, Queryable)]
#[diesel(table_name = crate::schema::deck)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Deck {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable, Serialize, Queryable)]
#[diesel(table_name = crate::schema::plugin)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Plugin {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable, Deserialize, Selectable, Serialize, Queryable)]
#[diesel(table_name = crate::schema::card)]
#[diesel(belongs_to(Deck), belongs_to(Plugin))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Card {
    pub id: i32,
    pub deck_id: i32,
    pub plugin_id: i32,
    pub plugin_name: String,
    pub plugin_data: serde_json::Value,
}

#[derive(Serialize)]
pub struct DeckWithCards {
    pub deck: Deck,
    pub card_ids: Vec<i32>,
}
