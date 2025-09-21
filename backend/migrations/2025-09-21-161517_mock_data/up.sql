-- Insert mock plugins
INSERT INTO plugin (name, http_address, tag) VALUES
('Basic Card Renderer', 'ourgit.flashqc/basic-renderer', 'renderer'),
('Image Card Plugin', 'ourgit.flashqc/image-plugin', 'image'),
('Math Formula Plugin', 'ourgit.flashqc/math-plugin', 'math'),
('Audio Player Plugin', 'ourgit.flashqc/audio-plugin', 'audio');

-- Insert mock users
INSERT INTO "user" (username, email, password_hash) VALUES
('john_doe', 'john@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj95.LuaoOfS'),
('jane_smith', 'jane@example.com', '$2b$12$EixZxQALz7N8z6J9C4Q8VeTN9U8v.7L8F8X4k2p9e3Q2h8t6s7x2'),
('study_master', 'study@example.com', '$2b$12$Q8F7x4k2p9e3Q2h8t6s7x2EixZxQALz7N8z6J9C4Q8VeTN9U8v.7'),
('language_learner', 'lang@example.com', '$2b$12$h8t6s7x2EixZxQALz7N8z6J9C4Q8VeTN9U8v.7L8F8X4k2p9e3Q2');

-- Insert mock flashcard templates
INSERT INTO flashcard_template (field_types) VALUES
('{"front": "text", "back": "text"}'),
('{"question": "text", "answer": "text", "hint": "text"}'),
('{"term": "text", "definition": "text", "example": "text"}'),
('{"image": "image", "caption": "text", "description": "text"}'),
('{"formula": "math", "explanation": "text", "variables": "text"}'),
('{"audio": "audio", "transcript": "text", "translation": "text"}');

-- Associate templates with plugins
INSERT INTO flashcard_template_plugin (template_id, plugin_id) VALUES
(1, 1), -- Basic template with basic renderer
(2, 1), -- Q&A template with basic renderer
(3, 1), -- Term definition with basic renderer
(4, 2), -- Image template with image plugin
(5, 3), -- Math template with math plugin
(6, 4); -- Audio template with audio plugin

-- Insert mock decks
INSERT INTO deck (name, description, plugin_id) VALUES
('Spanish Vocabulary', 'Essential Spanish words for beginners', 1),
('Math Formulas', 'Important mathematical formulas and equations', 3),
('Geography Facts', 'Countries, capitals, and geographical features', 1),
('Programming Concepts', 'Core programming terminology and concepts', 1),
('Audio Pronunciation', 'Language pronunciation practice', 4),
('Science Diagrams', 'Visual learning for science concepts', 2);

-- Insert mock flashcards
INSERT INTO flashcard (template_id, fields) VALUES
-- Spanish vocabulary cards (template 1)
(1, '{"front": "Hola", "back": "Hello"}'),
(1, '{"front": "Gracias", "back": "Thank you"}'),
(1, '{"front": "Adiós", "back": "Goodbye"}'),
(1, '{"front": "Por favor", "back": "Please"}'),

-- Q&A cards (template 2)
(2, '{"question": "What is the capital of France?", "answer": "Paris", "hint": "City of Light"}'),
(2, '{"question": "What is 2 + 2?", "answer": "4", "hint": "Basic arithmetic"}'),
(2, '{"question": "Who wrote Romeo and Juliet?", "answer": "William Shakespeare", "hint": "English playwright"}'),

-- Term definition cards (template 3)
(3, '{"term": "Algorithm", "definition": "A step-by-step procedure for solving a problem", "example": "Sorting numbers from smallest to largest"}'),
(3, '{"term": "Variable", "definition": "A storage location with an associated name", "example": "int age = 25;"}'),
(3, '{"term": "Function", "definition": "A reusable block of code that performs a specific task", "example": "def add(a, b): return a + b"}'),

-- Image cards (template 4)
(4, '{"image": "/images/eiffel_tower.jpg", "caption": "Eiffel Tower", "description": "Famous landmark in Paris, France"}'),
(4, '{"image": "/images/great_wall.jpg", "caption": "Great Wall of China", "description": "Ancient fortification in northern China"}'),

-- Math formula cards (template 5)
(5, '{"formula": "a² + b² = c²", "explanation": "Pythagorean theorem for right triangles", "variables": "a, b = legs; c = hypotenuse"}'),
(5, '{"formula": "E = mc²", "explanation": "Einstein''s mass-energy equivalence", "variables": "E = energy; m = mass; c = speed of light"}'),

-- Audio cards (template 6)
(6, '{"audio": "/audio/bonjour.mp3", "transcript": "Bonjour", "translation": "Hello (French)"}'),
(6, '{"audio": "/audio/guten_tag.mp3", "transcript": "Guten Tag", "translation": "Good day (German)"}');

-- Insert mock flashcard metadata (user progress)
INSERT INTO flashcard_metadata (ID_USER, ID_FLASHCARD, score) VALUES
-- User 1 (john_doe) progress
(1, 1, 5),
(1, 2, 3),
(1, 3, 4),
(1, 4, 2),
(1, 5, 5),

-- User 2 (jane_smith) progress
(2, 1, 4),
(2, 2, 5),
(2, 6, 3),
(2, 7, 4),
(2, 8, 2),

-- User 3 (study_master) progress
(3, 5, 5),
(3, 6, 5),
(3, 7, 4),
(3, 9, 5),
(3, 10, 4),

-- User 4 (language_learner) progress
(4, 1, 3),
(4, 2, 4),
(4, 3, 3),
(4, 13, 2),
(4, 14, 3);
