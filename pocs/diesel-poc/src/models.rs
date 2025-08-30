use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub uuid: Uuid,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::cards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Card {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::card_tags_link)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CardTagLink {
    pub card_id: i32,
    pub tag_id: i32,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::plugins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Plugin {
    pub id: i32,
    pub name: String,
    /// Either a local path or a URL (in the future most likely only URLs)
    /// This is the source and not the url where the plugin is accessible
    /// the plugin is downloaded from here
    pub link: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub id_plugin: i32,
}
