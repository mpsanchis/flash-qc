// @generated automatically by Diesel CLI.

diesel::table! {
    deck (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        deleted -> Bool,
        plugin_id -> Nullable<Int4>,
    }
}

diesel::table! {
    flashcard (id) {
        id -> Int4,
        version -> Int4,
        template_id -> Int4,
        deleted -> Bool,
        fields -> Jsonb,
    }
}

diesel::table! {
    flashcard_metadata (id_user, id_flashcard) {
        id_user -> Int4,
        id_flashcard -> Int4,
        score -> Int4,
    }
}

diesel::table! {
    flashcard_template (id) {
        id -> Int4,
        field_types -> Jsonb,
        deleted -> Bool,
    }
}

diesel::table! {
    flashcard_template_plugin (template_id, plugin_id) {
        template_id -> Int4,
        plugin_id -> Int4,
    }
}

diesel::table! {
    plugin (id) {
        id -> Int4,
        name -> Text,
        http_address -> Nullable<Text>,
        tag -> Nullable<Text>,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

diesel::joinable!(deck -> plugin (plugin_id));
diesel::joinable!(flashcard -> flashcard_template (template_id));
diesel::joinable!(flashcard_metadata -> flashcard (id_flashcard));
diesel::joinable!(flashcard_metadata -> user (id_user));
diesel::joinable!(flashcard_template_plugin -> flashcard_template (template_id));
diesel::joinable!(flashcard_template_plugin -> plugin (plugin_id));

diesel::allow_tables_to_appear_in_same_query!(
    deck,
    flashcard,
    flashcard_metadata,
    flashcard_template,
    flashcard_template_plugin,
    plugin,
    user,
);
