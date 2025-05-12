-- Add up migration script here
create table role
(
    id   varchar(255) primary key,
    name varchar(255) not null
);

create table user_assigned_roles
(
    user_id varchar(255),
    role_id varchar(255),
    foreign key (user_id) references users (id),
    foreign key (role_id) references role (id)
);