CREATE TABLE IF NOT EXISTS deck (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
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
    plugin_data JSONB NOT NULL DEFAULT '{}'
);

/* Bloody user word is reserved */
CREATE TABLE IF NOT EXISTS flashqc_user (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);
