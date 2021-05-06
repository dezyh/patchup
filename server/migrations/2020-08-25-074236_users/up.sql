create table users (
    id serial primary key,
    username varchar(64) not null unique,
    email varchar(128) not null,
    password varchar(256) not null,
    firstname varchar(64) not null,
    lastname varchar(64) not null,
    verified boolean default true not null,
    session varchar(256) not null default ''
);
