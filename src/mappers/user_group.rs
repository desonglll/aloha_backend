use crate::dto::pagination::Pagination;
use crate::dto::query::DtoQuery;
use crate::dto::response::DtoResponse;
use crate::models::user_group::UserGroup;
use anyhow::Context;
use sqlx::{Postgres, Transaction};
use tracing::error;
use uuid::Uuid;

pub async fn get_all_groups(
    mut transaction: Transaction<'_, Postgres>,
    dto_query: DtoQuery,
) -> Result<DtoResponse<Vec<UserGroup>>, anyhow::Error> {
    let offset = dto_query.offset() as i64;
    let limit = dto_query.size() as i64;
    let total = sqlx::query!("SELECT COUNT(*) FROM user_groups")
        .fetch_one(&mut *transaction)
        .await?
        .count;

    let data = sqlx::query_as!(
        UserGroup,
        "SELECT * FROM user_groups ORDER BY id LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch paginated user_groups")?;
    let pagination = Pagination::new(
        Option::from(dto_query.page()),
        Option::from(dto_query.size()),
        Option::from(total),
    );
    Ok(DtoResponse::new(data, Option::from(pagination)))
}

pub async fn get_group_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: Uuid,
) -> Result<UserGroup, anyhow::Error> {
    sqlx::query_as!(UserGroup, "select * from user_groups where id=$1", id)
        .fetch_one(&mut *transaction)
        .await
        .context("Failed to fetch user_groups")
}

pub async fn insert_user_group(
    mut transaction: Transaction<'_, Postgres>,
    group: &UserGroup,
) -> Result<UserGroup, anyhow::Error> {
    match sqlx::query_as!(
        UserGroup,
        "insert into user_groups (id, group_name) values ($1, $2) returning id, group_name",
        group.id,
        group.group_name
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to insert user_groups")
    {
        Ok(row) => {
            transaction
                .commit()
                .await
                .context("Failed to commit SQL transaction to insert a new user_group.")?;
            Ok(row)
        }
        Err(e) => Err(e),
    }
}

pub async fn delete_user_group_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: Uuid,
) -> Result<UserGroup, anyhow::Error> {
    match sqlx::query_as!(
        UserGroup,
        "delete from user_groups where id=$1 returning id, group_name",
        id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to delete user_groups")
    {
        Ok(row) => {
            transaction
                .commit()
                .await
                .context("Failed to commit SQL transaction to delete a user_group.")?;
            Ok(row)
        }
        Err(e) => {
            error!("Failed to delete user_groups from user_groups: {}", e);
            Err(e)
        }
    }
}

pub async fn update_user_group(
    mut transaction: Transaction<'_, Postgres>,
    group: &UserGroup,
) -> Result<UserGroup, anyhow::Error> {
    match sqlx::query_as!(
        UserGroup,
        "update user_groups set group_name = $1 where id = $2 returning id, group_name",
        group.group_name,
        group.id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to update user_groups")
    {
        Ok(row) => {
            transaction
                .commit()
                .await
                .context("Failed to commit SQL transaction to update a user_group.")?;
            Ok(row)
        }
        Err(e) => Err(e),
    }
}
