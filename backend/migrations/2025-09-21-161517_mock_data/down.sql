-- Remove mock data in reverse order of dependencies
DELETE FROM flashqc_user
WHERE id = 1;
DELETE FROM card
WHERE id IN (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
DELETE FROM plugin
WHERE id IN (1, 2, 3, 4);
DELETE FROM deck
WHERE id IN (1, 2, 3, 4);
