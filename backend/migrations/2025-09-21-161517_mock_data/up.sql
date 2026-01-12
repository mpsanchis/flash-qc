INSERT INTO deck (id, name) VALUES
(1, 'Da Vinci'),
(2, 'Memorization Deck'),
(3, 'Front & Back Deck'),
(4, 'Drawing Canvas'),
(5, 'Rubik''s Cube');

INSERT INTO plugin (id, name) VALUES
(1, 'flip-word'),
(2, 'dummy'),
(3, 'da-vinci-facts'),
(4, 'drawing-canvas'),
(5, 'rubiks-cube');

INSERT INTO card (id, deck_id, plugin_id, plugin_name, plugin_data, difficulty, stability, retrievability) VALUES
-- da vinci deck
(1, 1, 3, 'da-vinci-facts', '{"fact": "Da Vinci painted the Mona Lisa"}', NULL, NULL, NULL),
(2, 1, 3, 'da-vinci-facts', '{"fact": "Da Vinci was born on April 15, 1452"}', NULL, NULL, NULL),
(3, 1, 3, 'da-vinci-facts', '{"fact": "Da Vinci lived in the city of Florence"}', NULL, NULL, NULL),
-- Memorization deck
(4, 2, 2, 'dummy', '{ "word": "hello" }', NULL, NULL, NULL),
(5, 2, 2, 'dummy', '{ "word": "nonexistentword" }', NULL, NULL, NULL),
-- Front & Back Deck
(6, 3, 1, 'flip-word', '{"frontContent": "hello", "backContent": "world"}', NULL, NULL, NULL),
(7, 3, 1, 'flip-word', '{"frontContent": "Tom", "backContent": "Jerry"}', NULL, NULL, NULL),
(8, 3, 1, 'flip-word', '{"frontContent": "Batman", "backContent": "Robin"}', NULL, NULL, NULL),
-- Drawing Canvas
(9, 4, 4, 'drawing-canvas', '{"imageName": "ferris.png"}', NULL, NULL, NULL),
(10, 4, 4, 'drawing-canvas', '{"imageName": "ferris-wizard.png"}', NULL, NULL, NULL),
(11, 4, 4, 'drawing-canvas', '{"imageName": "ferris-viking.png"}', NULL, NULL, NULL),
-- Rubik's Cube
(12, 5, 5, 'rubiks-cube', '{"scramble": "R", "difficulty": "trivial"}', NULL, NULL, NULL),
(13, 5, 5, 'rubiks-cube', '{"scramble": "R U R'' U''", "difficulty": "beginner"}', NULL, NULL, NULL),
(14, 5, 5, 'rubiks-cube', '{"scramble": "F R U R'' U'' F''", "difficulty": "beginner"}', NULL, NULL, NULL),
(15, 5, 5, 'rubiks-cube', '{"scramble": "R U R'' U R U2 R''", "difficulty": "intermediate"}', NULL, NULL, NULL);

-- import user data
-- Password is "password" created manually with argon2
INSERT INTO flashqc_user (id, username, hashed_password, email) VALUES
(1, 'johndoe', '$argon2id$v=19$m=19456,t=2,p=1$MTIzNDU2Nzg$Zsmf7ZTiIiMUDRc6bF8sB2Xxyt1aG6hcw56qpR3xNJU', 'myemail@com');
