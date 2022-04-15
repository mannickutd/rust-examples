DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE todos (
    id bigserial primary key,
    name varchar(255),
    description varchar(255),
    date_added timestamp with time zone default CURRENT_TIMESTAMP,

    UNIQUE (name)
);