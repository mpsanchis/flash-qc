// @generated automatically by Diesel CLI.

diesel::table! {
    deck (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    plugincard (id) {
        id -> Int4,
        name -> Text,
        deck_id -> Int4,
    }
}

diesel::joinable!(plugincard -> deck (deck_id));

diesel::allow_tables_to_appear_in_same_query!(deck, plugincard,);
