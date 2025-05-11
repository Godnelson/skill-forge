-- Add down migration script here
drop table users;
create table users
(
    id        varchar(255) primary key,
    name      varchar(255) not null,
    bio       varchar(1000),
    pfp       varchar(255),
    cv        varchar(255),
    is_banned bit default 0
);