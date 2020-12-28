create table users (
    id serial primary key,
    username varchar(20) not null unique,
    email varchar(100) not null,
    password varchar(200) not null,
    firstname varchar(30) not null,
    lastname varchar(30) not null,
    verified boolean default true not null,
    session varchar(200) not null default ''
);