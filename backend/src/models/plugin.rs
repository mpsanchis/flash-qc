use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Insertable, Serialize, Queryable)]
#[diesel(table_name = crate::schema::plugin)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Plugin {
    pub id: i32,
    pub name: String,
}
