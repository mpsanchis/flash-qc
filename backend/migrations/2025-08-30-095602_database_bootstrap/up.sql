CREATE TABLE IF NOT EXISTS deck (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

-- Weird name "plugincard" on purpose, until we split plugins from cards (see: https://github.com/mpsanchis/flash-qc/milestone/3)
CREATE TABLE IF NOT EXISTS plugincard (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    deck_id INTEGER NOT NULL REFERENCES deck (id)
);
