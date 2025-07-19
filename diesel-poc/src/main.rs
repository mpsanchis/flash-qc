use diesel::prelude::*;
use flash_qc::{get_connection, models::*};
use rocket::response::{Responder, content::RawHtml};
use rocket::serde::json::Json;
use serde_json::json;
#[macro_use]
extern crate rocket;

use flash_qc::schema::{card_tags_link, cards, tags};

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct JsonWithHeaders {
    inner: Json<serde_json::Value>,
    cors_origin: rocket::http::Header<'static>,
}

impl JsonWithHeaders {
    fn new(json: Json<serde_json::Value>) -> Self {
        Self {
            inner: json,
            cors_origin: rocket::http::Header::new("Access-Control-Allow-Origin", "*"),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tags")]
fn tags_route() -> JsonWithHeaders {
    let connection = get_connection();
    let mut conn = connection.lock().unwrap();

    let results = tags::table
        .select(Tag::as_select())
        .load::<Tag>(&mut *conn)
        .expect("Error loading cards with tags");

    let mut current_tags: Vec<&Tag> = Vec::new();
    for tag in &results {
        current_tags.push(tag);
    }

    JsonWithHeaders::new(Json(json!({
        "tags": current_tags
    })))
}

#[get("/tags-html")]
fn tags_html() -> RawHtml<String> {
    let connection = get_connection();
    let mut conn = connection.lock().unwrap();

    let results = tags::table
        .select(Tag::as_select())
        .load::<Tag>(&mut *conn)
        .expect("Error loading tags");

    let mut html = String::from(
        "<!DOCTYPE html><html><head><title>Tags</title></head><body><h1>Tags</h1><ul>",
    );

    for tag in results {
        html.push_str(&format!("<li>{}</li>", tag.name));
    }

    html.push_str("</ul></body></html>");

    RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    let connection = get_connection();
    let mut conn = connection.lock().unwrap();

    let results = cards::table
        .left_join(card_tags_link::table.on(cards::id.eq(card_tags_link::card_id)))
        .left_join(tags::table.on(card_tags_link::tag_id.eq(tags::id)))
        .select((Card::as_select(), Option::<Tag>::as_select()))
        .load::<(Card, Option<Tag>)>(&mut *conn)
        .expect("Error loading cards with tags");

    // Group by card
    let mut current_card: Option<&Card> = None;
    let mut current_tags: Vec<&Tag> = Vec::new();

    for (card, tag) in &results {
        if current_card.map_or(true, |c| c.id != card.id) {
            // Print previous card if we have one
            if let Some(prev_card) = current_card {
                println!("Card: {}", prev_card.name);
                println!(
                    "Tags: {}",
                    if current_tags.is_empty() {
                        "None".to_string()
                    } else {
                        current_tags
                            .iter()
                            .map(|t| &t.name)
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ")
                    }
                );
                println!("-----------------------");
            }

            // Start new card
            current_card = Some(card);
            current_tags.clear();
        }

        if let Some(tag) = tag {
            current_tags.push(tag);
        }
    }

    // Print last card
    if let Some(card) = current_card {
        println!("Card: {}", card.name);
        println!(
            "Tags: {}",
            if current_tags.is_empty() {
                "None".to_string()
            } else {
                current_tags
                    .iter()
                    .map(|t| &t.name)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        );
        println!("-----------------------");
    }

    rocket::build().mount("/", routes![index, tags_route, tags_html, iframe_test])
}
