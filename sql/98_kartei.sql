CREATE TABLE card (
    id BIGSERIAL,
    question TEXT,
    answer TEXT,
    category INTEGER
);

SELECT * FROM card;

INSERT INTO card 
    (question, answer, category)
VALUES
    ('Hauptstadt Deutschland', 'Berlin', 1),
    ('Hauptstadt GB', 'London', 1),
    ('Hauptstadt Frankreich', 'Paris', 1),
    ('Sänger Band System of a Down', 'Serj Tankain', 2),
    ('Sänger Band Linkin Park', 'Chester Benington', 2);

Select question from card WHERE category = 2;