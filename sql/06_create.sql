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

