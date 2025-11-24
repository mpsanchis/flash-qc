use crate::schema::flashqc_user;
use crate::utils::db;
use crate::utils::gen_error::GenericError;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use base64::Engine;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{Route, post, routes as rocket_routes};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Queryable, Insertable, Serialize)]
#[diesel(table_name = flashqc_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
}

pub struct Auth {
    pub user: User,
}

/// Hashes using argon2, the salt is in the envvar SALT
pub fn hash_password(password: &str) -> String {
    let salt = std::env::var("SALT").unwrap_or_else(|_| "default_salt".into());
    let salt = SaltString::encode_b64(salt.as_bytes()).unwrap();

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    password_hash.to_string()
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = GenericError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut db_connection = db::get_connection().lock().unwrap();
        let unauthorized_response = Outcome::Error((
            Status::Unauthorized,
            GenericError::Custom("Unauthorized".into()),
        ));

        fn get_user_with_pass(
            db_connection: &mut diesel::pg::PgConnection,
            username: &str,
            password: &str,
        ) -> Result<User, diesel::result::Error> {
            let hashed_password = hash_password(password);
            println!(
                "Attempting to authenticate user: {}, with hashed password: {}",
                username, hashed_password
            );
            flashqc_user::table
                .select(flashqc_user::all_columns)
                .filter(flashqc_user::username.eq(username))
                .filter(flashqc_user::hashed_password.eq(hashed_password))
                .first(&mut *db_connection)
        }

        match req.headers().get_one("Authorization") {
            Some(header) if header.starts_with("Basic ") => {
                let username_password = &header[6..];
                let engine = base64::engine::general_purpose::STANDARD;
                let username_password = username_password.trim();
                let value_decoded = engine.decode(username_password);
                if value_decoded.is_err() {
                    return unauthorized_response;
                }
                let value_decoded = value_decoded.unwrap();
                let credentials = String::from_utf8(value_decoded);
                if credentials.is_err() {
                    return unauthorized_response;
                }
                let credentials = credentials.unwrap();
                let parts: Vec<&str> = credentials.splitn(2, ':').collect();
                println!("Username and password parts: {:?}", parts);
                if parts.len() != 2 {
                    return unauthorized_response;
                }
                let (username, password) = (parts[0], parts[1]);
                let user_obj = get_user_with_pass(&mut db_connection, username, password);
                if user_obj.is_err() {
                    return unauthorized_response;
                }
                let user_obj = user_obj.unwrap();
                Outcome::Success(Auth { user: user_obj })
            }
            Some(_) => unauthorized_response, // empty
            None => unauthorized_response,
        }
    }
}

#[post("/login")]
/// The _req is on purpose to trigger the FromRequest implementation
fn basic_auth(_req: Auth) -> Result<(), GenericError> {
    Ok(())
}

pub fn routes() -> Vec<Route> {
    rocket_routes![basic_auth]
}
