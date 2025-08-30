-- Your SQL goes here
ALTER TABLE tags
ADD COLUMN uuid UUID DEFAULT gen_random_uuid() NOT NULL;
