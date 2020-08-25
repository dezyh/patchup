create table users (
    username varchar(20) not null unique primary key,
    email varchar(100) not null unique,
    passhash varchar(200) not null,
    firstname varchar(30),
    lastname varchar(30),
    verified boolean default true,
    created_at timestamp not null
);