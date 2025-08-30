-- Your SQL goes here
CREATE TABLE IF NOT EXISTS deck(
  ID SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS flashcard_template (
  ID SERIAL PRIMARY KEY,
  -- field_types JSONB NOT NULL,
  fields JSONB NOT NULL,
  deleted BOOLEAN NOT NULL DEFAULT FALSE
  -- learning_parameters JSONB NOT NULL,
);

CREATE TABLE IF NOT EXISTS flashcard_instance (
  ID SERIAL PRIMARY KEY,
  template_id INTEGER NOT NULL REFERENCES flashcard_template(ID),
  deleted BOOLEAN NOT NULL DEFAULT FALSE,
  -- learning_progress JSONB NOT NULL,
  deck_id INTEGER NOT NULL REFERENCES deck(ID)
  -- user_id INTEGER NOT NULL REFERENCES users(ID)
);

