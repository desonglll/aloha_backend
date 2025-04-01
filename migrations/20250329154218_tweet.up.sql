-- Add migration script here
create table user_groups
(
    id         uuid primary key default gen_random_uuid(),
    group_name varchar(255) not null unique
);

-- insert into user_groups(group_name)
-- values ('admin');

create table "users"
(
    id            uuid primary key default gen_random_uuid(),
    username      varchar(255) not null unique,
    password_hash varchar(255) not null,
    created_at    timestamptz      default now(),
    user_group_id uuid,
    foreign key (user_group_id) references user_groups (id) on delete cascade
);

-- insert into "users"(username, password_hash, user_group_id)
-- values ('admin', 'admin', (select id from user_groups where group_name = 'admin' limit 1) );

create table tweet
(
    id         serial primary key,
    content    text,
    created_at timestamptz default now(),
    updated_at timestamptz default now(),
    user_id    uuid,
    foreign key (user_id) references "users" (id) on delete cascade
);

-- insert into tweet (content, user_id)
-- values ('This is a testing tweet, and time is ' || now(), (select id from users where username = 'admin' limit 1) );