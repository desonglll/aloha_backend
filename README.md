```shell
docker run --name aloha -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres

docker run --name aloha-redis -p 6379:6379 -d redis

sqlx database create

sqlx migrate run
sqlx migrate revert
```

## `.env`

```dotenv
DATABASE_URL=postgres://postgres:postgres@localhost:5432/aloha
```

## TODO

- [x] add user_group model
- [ ] add user model
- [ ] add tweet model


- [x] add mappers for user_group
- [ ] add mappers for user
- [ ] add mappers for tweet


- [ ] add CRUD for user_group
- [ ] add CRUD for user
- [ ] add CRUD for tweet


- [ ] test for user_group
- [ ] test for user
- [ ] test for tweet
