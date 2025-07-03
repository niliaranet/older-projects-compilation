drop table videos;
drop table users;

create table users (
    id serial primary key,
    username varchar(255),
    password varchar(255)
);

create table videos (
    id serial primary key,
    name varchar(255),
    description text,
    user_id int,
    foreign key (user_id) references users(id)
);

