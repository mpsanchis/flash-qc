// @generated automatically by Diesel CLI.

diesel::table! {
    card (id) {
        id -> Int4,
        deck_id -> Int4,
        plugin_id -> Int4,
        plugin_name -> Text,
        plugin_data -> Jsonb,
        difficulty -> Nullable<Float4>,
        stability -> Nullable<Float4>,
        retrievability -> Nullable<Float4>,
    }
}

diesel::table! {
    deck (id) {
        id -> Int4,
        name -> Text,
        desired_retention -> Float4,
        initial_stability_again_0 -> Float4,
        initial_stability_hard_1 -> Float4,
        initial_stability_good_2 -> Float4,
        initial_stability_easy_3 -> Float4,
        initial_difficulty_4 -> Float4,
        initial_difficulty_multiplier_5 -> Float4,
        difficulty_adjustment_6 -> Float4,
        difficulty_mean_regression_7 -> Float4,
        stability_exponent_8 -> Float4,
        stability_negative_power_9 -> Float4,
        stability_exponent_10 -> Float4,
        fail_stability_multiplier_11 -> Float4,
        fail_stability_negative_power_12 -> Float4,
        fail_stability_power_13 -> Float4,
        fail_stability_exponent_14 -> Float4,
        hard_stability_multiplier_15 -> Float4,
        easy_stability_multiplier_16 -> Float4,
        short_term_stability_exponent_17 -> Float4,
        short_term_stability_exponent_2_18 -> Float4,
        short_term_last_stability_exponent_19 -> Float4,
        interval_decay_factor_20 -> Float4,
    }
}

diesel::table! {
    flashqc_user (id) {
        id -> Int4,
        username -> Text,
        hashed_password -> Text,
        email -> Text,
    }
}

diesel::table! {
    plugin (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    training_event (id) {
        id -> Int4,
        card_id -> Int4,
        event_time -> Timestamp,
        result -> Float4,
    }
}

diesel::joinable!(card -> deck (deck_id));
diesel::joinable!(card -> plugin (plugin_id));
diesel::joinable!(training_event -> card (card_id));

diesel::allow_tables_to_appear_in_same_query!(card, deck, flashqc_user, plugin, training_event,);
