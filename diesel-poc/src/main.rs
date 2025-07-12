use diesel::prelude::*;
use flash_qc::{establish_connection, models::*};

fn main() {
    use flash_qc::schema::{cards, tags, card_tags_link};
    let connection = &mut establish_connection();

    let results = cards::table
        .left_join(card_tags_link::table.on(cards::id.eq(card_tags_link::card_id)))
        .left_join(tags::table.on(card_tags_link::tag_id.eq(tags::id)))
        .select((Card::as_select(), Option::<Tag>::as_select()))
        .load::<(Card, Option<Tag>)>(connection)
        .expect("Error loading cards with tags");

    // Group by card
    let mut current_card: Option<&Card> = None;
    let mut current_tags: Vec<&Tag> = Vec::new();
    
    for (card, tag) in &results {
        if current_card.map_or(true, |c| c.id != card.id) {
            // Print previous card if we have one
            if let Some(prev_card) = current_card {
                println!("Card: {}", prev_card.name);
                println!("Tags: {}", 
                    if current_tags.is_empty() { 
                        "None".to_string() 
                    } else { 
                        current_tags.iter().map(|t| &t.name).cloned().collect::<Vec<_>>().join(", ")
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
        println!("Tags: {}", 
            if current_tags.is_empty() { 
                "None".to_string() 
            } else { 
                current_tags.iter().map(|t| &t.name).cloned().collect::<Vec<_>>().join(", ")
            }
        );
        println!("-----------------------");
    }
}
