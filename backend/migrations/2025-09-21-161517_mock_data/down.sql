-- Remove mock data in reverse order of dependencies

-- Remove flashcard metadata
DELETE FROM flashcard_metadata WHERE ID_USER IN (1, 2, 3, 4);

-- Remove flashcards
DELETE FROM flashcard WHERE ID <= 14;

-- Remove flashcard template plugin associations
DELETE FROM flashcard_template_plugin WHERE template_id <= 6;

-- Remove decks
DELETE FROM deck WHERE name IN ('Spanish Vocabulary', 'Math Formulas', 'Geography Facts', 'Programming Concepts', 'Audio Pronunciation', 'Science Diagrams');

-- Remove flashcard templates
DELETE FROM flashcard_template WHERE ID <= 6;

-- Remove users
DELETE FROM "user" WHERE username IN ('john_doe', 'jane_smith', 'study_master', 'language_learner');

-- Remove plugins
DELETE FROM plugin WHERE name IN ('Basic Card Renderer', 'Image Card Plugin', 'Math Formula Plugin', 'Audio Player Plugin');
