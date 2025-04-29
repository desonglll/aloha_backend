
## Docker Ubuntu

```shell
docker run -itd --name my-ubuntu -p 8080:8080 -p 3000:3000 ubuntu /bin/bash # <host_port:container_port>

apt update && apt install git gcc curl pkg-config

# git config
git config --global user.name "xxx" && git config --global user.password "xxx" && git config --global user.email "xxx" && ssh-keygen -t ed25519 -C "xxx@163.com" && cat ~/.ssh/id_ed25519.pub

# install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# install sqlx-cli
cargo install --locked sqlx-cli

```

## Using local machine to run database

### [postgres-17](https://www.postgresql.org/download/linux/ubuntu/)

```shell
apt install curl ca-certificates
install -d /usr/share/postgresql-common/pgdg
curl -o /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc --fail https://www.postgresql.org/media/keys/ACCC4CF8.asc

. /etc/os-release
sh -c "echo 'deb [signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] https://apt.postgresql.org/pub/repos/apt $VERSION_CODENAME-pgdg main' > /etc/apt/sources.list.d/pgdg.list"
apt update

apt -y install postgresql-17

apt update
apt install -y locales
locale-gen en_US.UTF-8
update-locale LANG=en_US.UTF-8
export LANG=en_US.UTF-8


```

## Using docker to create database

```shell
docker run --name aloha -e POSTGRES_PASSWORD=postgres -e POSTGRES_USER=postgres -p 5432:5432 -d postgres
docker run --name aloha-redis -p 6379:6379 -d redis
```



```shell
sqlx database create
sqlx migrate run
sqlx migrate revert
```

## `.env`

```dotenv
DATABASE_URL=postgres://postgres:postgres@localhost:5432/aloha
RUST_LOG=debug
ALOHA_ENVIRONMENT=development# production
```

## TODO

- [x] add user_group model
- [ ] add user model
- [ ] add tweet model


- [x] add mappers for user_group
- [ ] add mappers for user
- [ ] add mappers for tweet


- [x] add CRUD for user_group
- [ ] add CRUD for user
- [ ] add CRUD for tweet


- [ ] test for user_group
- [ ] test for user
- [ ] test for tweet

- [x] fix for user_group
- [x] fix for permission
- [x] fix for group_permission
- [x] fix for user
- [ ] fix for tweet


- [ ] change user password String to hashed password_hash in production mode.