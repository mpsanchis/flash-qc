// @generated automatically by Diesel CLI.

diesel::table! {
    card_tags_link (card_id, tag_id) {
        card_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    cards (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        uuid -> Uuid,
    }
}

diesel::joinable!(card_tags_link -> cards (card_id));
diesel::joinable!(card_tags_link -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(card_tags_link, cards, tags,);
