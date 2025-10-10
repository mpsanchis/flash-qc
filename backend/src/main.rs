use models::{Deck, FlashcardInstance, FlashcardTemplate};
use rocket::launch;
extern crate rocket;

mod models;
mod schema;
mod system;
mod root;

#[launch]
fn rocket() -> _ {
    let deck = Deck {
        id: 1,
        name: "Sample Deck".to_string(),
        description: Some("This is a sample deck.".to_string()),
        deleted: false,
    };

    let template = FlashcardTemplate {
        id: 1,
        fields: serde_json::json!({"question": "What is Rust?", "answer": "A programming language."}),
        deleted: false,
    };

    let instance = FlashcardInstance {
        id: 1,
        template_id: template.id,
        deleted: false,
        deck_id: deck.id,
    };

    println!("Deck: {:?}", deck);
    println!("Template: {:?}", template);
    println!("Instance: {:?}", instance);

    rocket::build()
      .attach(system::Cors)
      .mount("/", root::routes())
      .mount("/system", system::routes())
}
