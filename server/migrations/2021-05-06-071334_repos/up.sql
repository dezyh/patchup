create table repos (
    id serial primary key,
    creator integer not null,
    manager integer,
    created timestamp not null default now(),
    description varchar(512) not null default '',

    foreign key (creator) references users(id),
    foreign key (manager) references orgs(id)
);
