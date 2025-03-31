
```shell
docker run --name aloha -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres

docker run --name aloha-redis -p 6379:6379 -d redis

sqlx database create

sqlx migrate run
sqlx migrate revert
```