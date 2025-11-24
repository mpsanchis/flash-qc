use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::serde::json::json;
use serde::Serialize;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Error;
use thiserror::Error;

fn serialize_error<E, S>(error: &E, serializer: S) -> Result<S::Ok, S::Error>
where
    E: Display,
    S: serde::Serializer,
{
    serializer.serialize_str(&error.to_string())
}

#[derive(Error, Debug, Serialize)]
pub enum GenericError {
    #[error("IO Error: {0}")]
    #[serde(serialize_with = "serialize_error")]
    Io(#[from] Error),
    #[error("Rocket JSON Error: {0}")]
    #[serde(serialize_with = "serialize_error")]
    Custom(String),
}

impl<'a> From<rocket::serde::json::Error<'a>> for GenericError {
    fn from(err: rocket::serde::json::Error<'a>) -> Self {
        GenericError::Custom(format!("Rocket JSON Error: {}", err))
    }
}

impl<'r> Responder<'r, 'static> for GenericError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let error_message = json!({
            "status": "error",
            "message": self.to_string()
        });
        Response::build()
            .sized_body(
                error_message.to_string().len(),
                Cursor::new(error_message.to_string()),
            )
            .status(Status::BadRequest)
            .ok()
    }
}
