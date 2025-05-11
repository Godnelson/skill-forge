-- Add up migration script here
drop table user_skill;
drop table review;
drop table users;

create table if not exists users
(
    id        varchar(255) primary key,
    name      varchar(255) not null,
    password  varchar(255) not null,
    email     varchar(255) not null unique,
    bio       varchar(1000),
    pfp       varchar(255),
    cv        varchar(255),
    is_banned bool default false
);

create table if not exists user_skill
(
    id      varchar(255) primary key,
    name    varchar(255) not null,
    user_id varchar(255),
    foreign key (user_id) references users (id)
);

create table if not exists review
(
    id               varchar(255) primary key,
    rate             float check (rate >= 0 and rate <= 5),
    description      varchar(1000),
    user_sender_id   varchar(255),
    user_receiver_id varchar(255),
    foreign key (user_sender_id) references users (id),
    foreign key (user_sender_id) references users (id)
);