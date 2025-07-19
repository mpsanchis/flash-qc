CREATE TABLE plugins (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    link TEXT NOT NULL
);
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    id_plugin INTEGER NOT NULL REFERENCES plugins(id)
);
