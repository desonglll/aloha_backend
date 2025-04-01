use crate::models::user_group::UserGroup;
use anyhow::Context;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_all_groups(
    mut transaction: Transaction<'_, Postgres>,
) -> Result<Vec<UserGroup>, sqlx::Error> {
    sqlx::query_as!(UserGroup, "select * from user_groups")
        .fetch_all(&mut *transaction)
        .await
}

pub async fn get_group_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: Uuid,
) -> Result<UserGroup, sqlx::Error> {
    sqlx::query_as!(UserGroup, "select * from user_groups where id=$1", id)
        .fetch_one(&mut *transaction)
        .await
}

pub async fn insert_user_group(
    mut transaction: Transaction<'_, Postgres>,
    group: &UserGroup,
) -> Result<UserGroup, sqlx::Error> {
    let inserted_group = sqlx::query_as!(
        UserGroup,
        "insert into user_groups (id, group_name) values ($1, $2) returning id, group_name",
        group.id,
        group.group_name
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to insert user_groups")
    .unwrap();
    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to store a new subscriber.")
        .unwrap();
    Ok(UserGroup {
        id: inserted_group.id,
        group_name: inserted_group.group_name,
    })
}

pub async fn delete_user_group_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: Uuid,
) -> Result<UserGroup, sqlx::Error> {
    sqlx::query_as!(
        UserGroup,
        "delete from user_groups where id=$1 returning id, group_name",
        id
    )
    .fetch_one(&mut *transaction)
    .await
}

pub async fn update_user_group(
    mut transaction: Transaction<'_, Postgres>,
    group: &UserGroup,
) -> Result<UserGroup, sqlx::Error> {
    sqlx::query_as!(
        UserGroup,
        "update user_groups set group_name = $1 where id = $2 returning id, group_name",
        group.group_name,
        group.id
    )
    .fetch_one(&mut *transaction)
    .await
}
