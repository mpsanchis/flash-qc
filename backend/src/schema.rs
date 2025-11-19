// @generated automatically by Diesel CLI.

diesel::table! {
    card (id) {
        id -> Int4,
        deck_id -> Int4,
        plugin_id -> Int4,
        plugin_name -> Text,
        plugin_data -> Jsonb,
    }
}

diesel::table! {
    deck (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    plugin (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::joinable!(card -> deck (deck_id));
diesel::joinable!(card -> plugin (plugin_id));

diesel::allow_tables_to_appear_in_same_query!(
    card,
    deck,
    plugin,
);
