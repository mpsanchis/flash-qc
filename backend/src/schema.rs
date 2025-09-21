// @generated automatically by Diesel CLI.

diesel::table! {
    deck (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        deleted -> Bool,
    }
}

diesel::table! {
    flashcard_instance (id) {
        id -> Int4,
        template_id -> Int4,
        deleted -> Bool,
        deck_id -> Int4,
    }
}

diesel::table! {
    flashcard_template (id) {
        id -> Int4,
        fields -> Jsonb,
        deleted -> Bool,
    }
}

diesel::joinable!(flashcard_instance -> deck (deck_id));
diesel::joinable!(flashcard_instance -> flashcard_template (template_id));

diesel::allow_tables_to_appear_in_same_query!(deck, flashcard_instance, flashcard_template,);
