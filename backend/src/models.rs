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
}
#[derive(Debug, Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::flashcard_template)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FlashcardTemplate {
    pub id: i32,
    pub fields: serde_json::Value,
    pub deleted: bool,
}

#[derive(Debug, Insertable, Deserialize, Serialize, Queryable)]
#[diesel(table_name = crate::schema::flashcard_instance)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FlashcardInstance {
    pub id: i32,
    pub template_id: i32,
    pub deleted: bool,
    pub deck_id: i32,
}
