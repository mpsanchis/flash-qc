use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use super::deck::Deck;
#[allow(unused_imports)]
use super::plugin::Plugin;

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
    pub difficulty: Option<f32>,
    pub retrievability: Option<f32>,
    /// Amount of days which takes for retrievability to go from 100% to 90
    pub stability: Option<f32>,
}
