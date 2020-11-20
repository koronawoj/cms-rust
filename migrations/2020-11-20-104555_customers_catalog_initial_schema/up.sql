CREATE TABLE customers (
    id BIGSERIAL PRIMARY KEY,
    first_name varchar(256) NOT NULL,
    last_name varchar(256) NOT NULL,
    email varchar(256) NOT NULL,
    address varchar(256) NOT NULL
);