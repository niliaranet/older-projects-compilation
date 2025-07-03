create extension pg_trgm;

create database jirafeitor;
create user jirafeitor_user with password 'changeme';

\connect jirafeitor;

grant all privileges on database jirafeitor to jirafeitor_user;

drop table videos;
drop table users;

create table users (
    id serial primary key,
    username varchar(255) unique,
    password varchar(255)
);

create table videos (
    id serial primary key,
    name varchar(255),
    description text,
    user_id int,
    foreign key (user_id) references users(id)
);

grant usage, select on all sequences in schema public TO jirafeitor_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA public 
GRANT USAGE, SELECT ON SEQUENCES TO jirafeitor_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE videos TO jirafeitor_user;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO jirafeitor_user;
