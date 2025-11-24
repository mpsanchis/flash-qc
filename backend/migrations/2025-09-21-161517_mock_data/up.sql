INSERT INTO deck (id, name) VALUES
(1, 'Da Vinci'),
(2, 'Memorization Deck');
-- (3, 'Arts & Crafts Deck');

INSERT INTO plugin (id, name) VALUES
(1, 'hello-world-js'),
(2, 'dummy'),
(3, 'da-vinci-facts'),
(4, 'drawing-canvas');

INSERT INTO card (id, deck_id, plugin_id, plugin_name, plugin_data) VALUES
(1, 1, 3, 'da-vinci-facts', '{"fact": "Da Vinci painted the Mona Lisa"}'),
(2, 1, 3, 'da-vinci-facts', '{"fact": "Da Vinci was born on April 15, 1452"}'),
(3, 1, 3, 'da-vinci-facts', '{"fact": "Da Vinci lived in the city of Florence"}'),
(4, 2, 2, 'dummy', '{ "word": "hello" }'),
(5, 2, 2, 'dummy', '{ "word": "nonexistentword" }');
-- (1, 1, 'hello-world-js'),
-- (2, 1, 'dummy'),
-- (4, 2, 'drawing-canvas');