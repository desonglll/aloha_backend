use crate::models::user_group::UserGroup;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_group_by_id(pool: &PgPool, id: Uuid) -> Result<UserGroup, sqlx::Error> {
    sqlx::query_as!(UserGroup, "select * from user_groups where id=$1", id)
        .fetch_one(pool)
        .await
}

pub async fn insert_user_group(pool: &PgPool, group: &UserGroup) -> Result<UserGroup, sqlx::Error> {
    sqlx::query_as!(
        UserGroup,
        "insert into user_groups (id, group_name) values ($1, $2) returning id, group_name",
        group.id,
        group.group_name
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_user_group_by_id(pool: &PgPool, id: Uuid) -> Result<UserGroup, sqlx::Error> {
    sqlx::query_as!(
        UserGroup,
        "delete from user_groups where id=$1 returning id, group_name",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_user_group(pool: &PgPool, group: &UserGroup) -> Result<UserGroup, sqlx::Error> {
    sqlx::query_as!(
        UserGroup,
        "update user_groups set group_name = $1 where id = $2 returning id, group_name",
        group.group_name,
        group.id
    )
    .fetch_one(pool)
    .await
}
