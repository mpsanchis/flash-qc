use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::sync::{Mutex, OnceLock};

static DB_CONNECTION: OnceLock<Mutex<PgConnection>> = OnceLock::new();

pub fn get_connection() -> &'static Mutex<PgConnection> {
    DB_CONNECTION.get_or_init(|| {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Mutex::new(connection)
    })
}
