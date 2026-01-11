INSERT INTO deck (id, name) VALUES (5, 'Rubik''s Cube');

INSERT INTO plugin (id, name) VALUES (5, 'rubiks-cube');

INSERT INTO card (id, deck_id, plugin_id, plugin_name, plugin_data) VALUES
(12, 5, 5, 'rubiks-cube', '{"scramble": "R", "difficulty": "trivial"}'),
(13, 5, 5, 'rubiks-cube', '{"scramble": "R U R'' U''", "difficulty": "beginner"}'),
(14, 5, 5, 'rubiks-cube', '{"scramble": "F R U R'' U'' F''", "difficulty": "beginner"}'),
(15, 5, 5, 'rubiks-cube', '{"scramble": "R U R'' U R U2 R''", "difficulty": "intermediate"}');
