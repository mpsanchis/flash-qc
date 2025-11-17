use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable, Deserialize, Selectable, Serialize, Queryable)]
#[diesel(table_name = crate::schema::deck)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Deck {
    pub id: i32,
    pub name: String,
}

#[derive(Associations, Debug, Deserialize, Insertable, Serialize, Queryable)]
#[diesel(table_name = crate::schema::plugincard)]
#[diesel(belongs_to(Deck))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PluginCard {
    pub id: i32,
    pub name: String,
    pub deck_id: i32,
}
