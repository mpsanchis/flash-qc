-- Your SQL goes here
CREATE TABLE IF NOT EXISTS plugin(
  ID SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  http_address TEXT,
  tag TEXT
);

CREATE TABLE IF NOT EXISTS "user"(
  ID SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS deck(
  ID SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  deleted BOOLEAN NOT NULL DEFAULT FALSE,
  plugin_id INTEGER REFERENCES plugin(ID)
);

CREATE TABLE IF NOT EXISTS flashcard_template (
  ID SERIAL PRIMARY KEY,
  field_types JSONB NOT NULL,
  deleted BOOLEAN NOT NULL DEFAULT FALSE
);

COMMENT ON COLUMN flashcard_template.field_types IS 'A JSONB object where keys are field names and values are field types (e.g., "text", "image", etc.)';

CREATE TABLE IF NOT EXISTS flashcard_template_plugin (
  template_id INTEGER NOT NULL REFERENCES flashcard_template(ID),
  plugin_id INTEGER NOT NULL REFERENCES plugin(ID),
  PRIMARY KEY (template_id, plugin_id)
);

COMMENT ON TABLE flashcard_template_plugin IS 'Associates flashcard templates with plugins that can render or process them';

CREATE TABLE IF NOT EXISTS flashcard (
  ID SERIAL PRIMARY KEY,
  VERSION INTEGER NOT NULL DEFAULT 1,
  template_id INTEGER NOT NULL REFERENCES flashcard_template(ID),
  deleted BOOLEAN NOT NULL DEFAULT FALSE,
  fields JSONB NOT NULL
);

COMMENT ON COLUMN flashcard.VERSION IS 'Incremented each time the flashcard is updated';

CREATE TABLE IF NOT EXISTS flashcard_metadata (
  id_user INTEGER NOT NULL REFERENCES "user"(ID),
  id_flashcard INTEGER NOT NULL REFERENCES flashcard(ID),
  score INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (ID_USER, ID_FLASHCARD)
);

COMMENT ON COLUMN flashcard_metadata.score IS 'This is a simplified score for now';

