create table patches (
    id serial primary key,
    repo integer not null,
    created timestamp not null default now(),
    
    downloads integer not null default 0,
    bytesize integer,
    platform varchar(64),
    source_version varchar(64) not null,
    target_version varchar(64) not null,
    data varchar(256),

    foreign key (repo) references repos(id)
);
