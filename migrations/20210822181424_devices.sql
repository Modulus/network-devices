-- Add migration script here
create DATABASE devices;

create table devices(
    name varchar primary key,
    domain varchar,
    address varchar,
    icon varchar,
    comment varchar null
)