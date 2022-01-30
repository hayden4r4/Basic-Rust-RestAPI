-- Your SQL goes here
CREATE TABLE testtable (
    id BIGSERIAL PRIMARY KEY,
    timestamp DATE NOT NULL,
    symbol VARCHAR NOT NULL,
    price FLOAT4 NOT NULL
);