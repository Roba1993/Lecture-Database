/* Create the table */
CREATE TABLE users (
    id BIGSERIAL,
    firstname TEXT,
    lastname TEXT,
    age INTEGER,
    student BOOLEAN
);

/* Insert test data */
INSERT INTO users
(firstname, lastname, age, student)
VALUES
('Robert', 'Schuette', 30, false),
('Elke', 'Hofman', 33, true);

/* Read the data */
SELECT * FROM users;

/* Change test data */
1;UPDATE users SET age=222 WHERE id=1;

