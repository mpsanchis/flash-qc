CREATE TABLE IF NOT EXISTS deck (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    desired_retention REAL NOT NULL DEFAULT 0.9,
    initial_stability_again_0 REAL NOT NULL DEFAULT 0.212,
    initial_stability_hard_1 REAL NOT NULL DEFAULT 1.2931,
    initial_stability_good_2 REAL NOT NULL DEFAULT 2.3065,
    initial_stability_easy_3 REAL NOT NULL DEFAULT 8.2956,
    initial_difficulty_4 REAL NOT NULL DEFAULT 6.4133,
    initial_difficulty_multiplier_5 REAL NOT NULL DEFAULT 0.8334,
    difficulty_adjustment_6 REAL NOT NULL DEFAULT 3.0194,
    difficulty_mean_regression_7 REAL NOT NULL DEFAULT 0.001,
    stability_exponent_8 REAL NOT NULL DEFAULT 1.8722,
    stability_negative_power_9 REAL NOT NULL DEFAULT 0.0614,
    stability_exponent_10 REAL NOT NULL DEFAULT 0.796,
    fail_stability_multiplier_11 REAL NOT NULL DEFAULT 1.4835,
    fail_stability_negative_power_12 REAL NOT NULL DEFAULT 0.0614,
    fail_stability_power_13 REAL NOT NULL DEFAULT 0.2629,
    fail_stability_exponent_14 REAL NOT NULL DEFAULT 1.6483,
    hard_stability_multiplier_15 REAL NOT NULL DEFAULT 0.6014,
    easy_stability_multiplier_16 REAL NOT NULL DEFAULT 1.8729,
    short_term_stability_exponent_17 REAL NOT NULL DEFAULT 0.5425,
    short_term_stability_exponent_2_18 REAL NOT NULL DEFAULT 0.0912,
    short_term_last_stability_exponent_19 REAL NOT NULL DEFAULT 0.0658,
    interval_decay_factor_20 REAL NOT NULL DEFAULT 0.1542
);

CREATE TABLE IF NOT EXISTS plugin (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS card (
    id SERIAL PRIMARY KEY,
    deck_id INTEGER NOT NULL REFERENCES deck (id),
    plugin_id INTEGER NOT NULL REFERENCES plugin (id),
    plugin_name TEXT NOT NULL,
    plugin_data JSONB NOT NULL DEFAULT '{}',
    difficulty REAL,
    stability REAL,
    retrievability REAL
);

/* Bloody user word is reserved */
CREATE TABLE IF NOT EXISTS flashqc_user (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS training_event (
    id SERIAL PRIMARY KEY,
    card_id INTEGER NOT NULL REFERENCES card (id),
    event_time TIMESTAMP NOT NULL DEFAULT NOW(),
    result REAL NOT NULL CHECK (result >= 1.0 AND result <= 4.0)
);
