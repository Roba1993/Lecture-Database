/* Create the users table */
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    firstname TEXT,
    lastname TEXT,
    birthday TIMESTAMP,
    student BOOLEAN
);

/* Create the lecture table */
CREATE TABLE lecture (
    id BIGSERIAL PRIMARY KEY,
    name TEXT
);

/* Create the reference table */
CREATE TABLE user_lecture (
    user_id BIGSERIAL REFERENCES users (id),
    lecture_id BIGSERIAL REFERENCES lecture (id)
);

/* View table schema */
\d user_lecture

INSERT INTO lecture 
(name) 
VALUES
('Datenbanken'),
('It-Security'),
('Mathe');

INSERT INTO user_lecture 
(user_id, lecture_id) 
VALUES
(1, 1),
(1,3),
(2,1);

INSERT INTO users 
(firstname, lastname, birthday, student) 
VALUES
('Flo', 'Hamer', now(), true);

INSERT INTO lecture 
(name) 
VALUES
('Englisch');