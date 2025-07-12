use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
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
