create table permissions
(
    id          uuid primary key default gen_random_uuid(),
    name        varchar(255) not null unique,
    description text,
    created_at  timestamptz default now()
);

-- Add index on name for faster lookups
create index idx_permissions_name on permissions(name);

create table user_groups
(
    id         uuid primary key default gen_random_uuid(),
    group_name varchar(255) not null unique,
    created_at timestamptz default now()
);

-- Add index on group_name for faster lookups
create index idx_user_groups_name on user_groups(group_name);

create table group_permissions
(
    group_id      uuid not null,
    permission_id uuid not null,
    created_at    timestamptz default now(),
    primary key (group_id, permission_id),
    foreign key (group_id) references user_groups (id) on delete cascade,
    foreign key (permission_id) references permissions (id) on delete cascade
);

-- Add indexes for foreign keys to improve join performance
create index idx_group_permissions_group_id on group_permissions(group_id);
create index idx_group_permissions_permission_id on group_permissions(permission_id);

create table "users"
(
    id            uuid primary key default gen_random_uuid(),
    username      varchar(255) not null unique,
    password_hash varchar(255) not null,
    created_at    timestamptz default now(),
    user_group_id uuid,
    foreign key (user_group_id) references user_groups (id) on delete set null
);

-- Add indexes for foreign keys and frequently queried fields
create index idx_users_username on users(username);
create index idx_users_user_group_id on users(user_group_id);

create table user_permissions
(
    user_id       uuid not null,
    permission_id uuid not null,
    created_at    timestamptz default now(),
    primary key (user_id, permission_id),
    foreign key (user_id) references "users" (id) on delete cascade,
    foreign key (permission_id) references permissions (id) on delete cascade
);

-- Add indexes for foreign keys to improve join performance
create index idx_user_permissions_user_id on user_permissions(user_id);
create index idx_user_permissions_permission_id on user_permissions(permission_id);

create table tweet
(
    id         serial primary key,
    content    text not null,
    created_at timestamptz default now(),
    updated_at timestamptz default now(),
    user_id    uuid not null,
    foreign key (user_id) references "users" (id) on delete cascade
);

-- Add indexes for foreign keys and frequently queried fields
create index idx_tweet_user_id on tweet(user_id);
create index idx_tweet_created_at on tweet(created_at);

-- Add trigger to automatically update the updated_at column
create or replace function update_updated_at_column()
returns trigger as $$
begin
    new.updated_at = now();
    return new;
end;
$$ language plpgsql;

create trigger update_tweet_updated_at
before update on tweet
for each row
execute function update_updated_at_column();

