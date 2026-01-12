use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use super::card::Card;

#[derive(Debug, Insertable, Deserialize, Selectable, Serialize, Queryable)]
#[diesel(table_name = crate::schema::training_event)]
#[diesel(belongs_to(Card))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TrainingEvent {
    pub id: i32,
    pub card_id: i32,
    pub event_time: chrono::NaiveDateTime,
    pub result: f32,
}
