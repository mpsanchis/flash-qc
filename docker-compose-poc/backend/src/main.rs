#[macro_use] extern crate rocket;

use rocket::response::{content, status};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::{Request, Response};
use std::env;
use tokio_postgres::{NoTls, Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    created_at: String,
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> content::RawJson<&'static str> {
    content::RawJson(r#"{"message": "Hello from Rocket backend!", "status": "running"}"#)
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello from Rocket backend!"
}

async fn test_db_connection(database_url: &str) -> bool {
    match tokio_postgres::connect(database_url, NoTls).await {
        Ok((client, connection)) => {
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Database connection error: {}", e);
                }
            });

            match client.query("SELECT 1", &[]).await {
                Ok(_) => true,
                Err(e) => {
                    eprintln!("Database query error: {}", e);
                    false
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            false
        }
    }
}

#[get("/health")]
async fn health() -> content::RawJson<String> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@postgres:5432/mydb".to_string());

    let db_status = if test_db_connection(&database_url).await {
        "connected"
    } else {
        "disconnected"
    };

    let response = format!(r#"{{"status": "healthy", "database": "{}"}}"#, db_status);
    content::RawJson(response)
}

#[get("/api_hello")]
fn api_hello() -> &'static str {
    "Hello from Rocket API!"
}

async fn get_database_connection() -> Result<Client, tokio_postgres::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@postgres:5432/mydb".to_string());

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}

#[get("/users")]
async fn get_users() -> Result<content::RawJson<String>, status::Custom<String>> {
    match get_database_connection().await {
        Ok(client) => {
            match client.query("SELECT id, name, email, created_at::TEXT FROM users ORDER BY id", &[]).await {
                Ok(rows) => {
                    let mut users = Vec::new();
                    for row in rows {
                        let user = User {
                            id: row.get(0),
                            name: row.get(1),
                            email: row.get(2),
                            created_at: row.get::<_, String>(3),
                        };
                        users.push(user);
                    }
                    match serde_json::to_string(&users) {
                        Ok(json) => Ok(content::RawJson(json)),
                        Err(_) => Result::Err(status::Custom(Status::InternalServerError,  String::from("Failed to serialize users")))
                    }
                },
                Err(_) => Result::Err(status::Custom(Status::InternalServerError,  String::from("Failed to query users")))
            }
        },
        Err(_) => Result::Err(status::Custom(Status::InternalServerError, String::from("Failed to connect to database")))
    }
}

#[launch]
fn rocket() -> _ {
    println!("Starting Rocket backend...");

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@postgres:5432/mydb".to_string());

    println!("Database URL: {}", database_url);

    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, hello, health, api_hello, get_users])
        .configure(rocket::Config::figment()
            .merge(("address", "0.0.0.0"))
            .merge(("port", 8000))
        )
}