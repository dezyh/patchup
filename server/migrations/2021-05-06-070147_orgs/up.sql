create table orgs (
    id serial primary key,
    tag varchar(64) not null,
    name varchar(64) not null
);

create table orgs_users (
    id serial primary key,
    org_id integer not null,
    user_id integer not null,

    foreign key (org_id) references orgs(id),
    foreign key (user_id) references users(id)
);
