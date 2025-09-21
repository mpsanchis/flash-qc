use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable, Deserialize, Serialize, Queryable)]
#[diesel(table_name = crate::schema::deck)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Deck {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub deleted: bool,
    pub plugin_id: Option<i32>,
}

#[derive(Debug, Insertable, Deserialize, Serialize, Queryable)]
#[diesel(table_name = crate::schema::flashcard)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FlashcardInstance {
    pub id: i32,
    pub template_id: i32,
    pub deleted: bool,
    pub fields: serde_json::Value,
}
