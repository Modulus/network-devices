-- Add migration script here

create table devices(
    name varchar primary key,
    domain varchar,
    address varchar,
    icon varchar,
    comment varchar null
)