/* Create the table */
CREATE TABLE users (
    id BIGSERIAL,
    firstname TEXT,
    lastname TEXT,
    age INTEGER,
    student BOOLEAN
);

/* View table schema */
\d users