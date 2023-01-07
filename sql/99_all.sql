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
('Robert', 'Schütte', 30, false),
('Elke', 'Hofman', 33, true);

/* Read the data */
SELECT * FROM users;

/* Read the data for people over 30 */
SELECT * FROM users WHERE age > 30;

