/* Create the table with uniqeness*/
CREATE TABLE users (
    id BIGSERIAL UNIQUE,
    firstname TEXT,
    lastname TEXT,
    age INTEGER,
    student BOOLEAN,
    UNIQUE(firstname, lastname, age)
);

/* Add uniqness later to a coloumn */
ALTER TABLE users ADD UNIQUE (firstname, lastname, age);