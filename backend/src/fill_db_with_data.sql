-- Insert three flashcard templates
INSERT INTO flashcard_template (id, fields, deleted) VALUES
  (1, '{"front": "What is the capital of France?", "back": "Paris"}', false),
  (2, '{"front": "What is 2 + 2?", "back": "4"}', false),
  (3, '{"front": "What is the largest planet in our solar system?", "back": "Jupiter"}', false);

-- Insert a deck
INSERT INTO deck (id, name, description, deleted) VALUES
  (1, 'General Knowledge', 'A deck for general knowledge questions.', false);

-- Insert three flashcard instances, one per template
INSERT INTO flashcard_instance (id, template_id, deleted, deck_id) VALUES
  (1, 1, false, 1),
  (2, 2, false, 1),
  (3, 3, false, 1);
