-- Your SQL goes here
CREATE TABLE testtable (
    id SERIAL PRIMARY KEY,
    timestamp DATE NOT NULL,
    symbol VARCHAR NOT NULL,
    price FLOAT NOT NULL
);