use crate::models::user_group::UserGroup;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_group_by_id(pool: &PgPool, id: Uuid) -> Result<UserGroup, sqlx::Error> {
    sqlx::query_as!(UserGroup, "select * from user_groups where id=$1", id)
        .fetch_one(pool)
        .await
}
