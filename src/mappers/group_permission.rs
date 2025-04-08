use crate::dto::pagination::Pagination;
use crate::dto::query::DtoQuery;
use crate::dto::response::DtoResponse;
use crate::models::group_permission::GroupPermission;
use anyhow::{Context, Result};
use sqlx::{Postgres, Transaction};
use tracing::error;
use uuid::Uuid;

pub async fn get_all_group_permissions(
    mut transaction: Transaction<'_, Postgres>,
    dto_query: DtoQuery,
) -> Result<DtoResponse<Vec<GroupPermission>>, anyhow::Error> {
    let offset = dto_query.offset() as i64;
    let limit = dto_query.size() as i64;
    let total = sqlx::query!("SELECT COUNT(*) FROM group_permissions")
        .fetch_one(&mut *transaction)
        .await?
        .count;

    let data = sqlx::query_as!(
        GroupPermission,
        "SELECT * FROM group_permissions ORDER BY group_id, permission_id LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch paginated group_permissions")?;

    let pagination = Pagination::new(
        Option::from(dto_query.page()),
        Option::from(dto_query.size()),
        total,
    );
    Ok(DtoResponse::new(data, Option::from(pagination)))
}

pub async fn get_group_permissions_by_group_id(
    mut transaction: Transaction<'_, Postgres>,
    group_id: Uuid,
) -> Result<Vec<GroupPermission>, anyhow::Error> {
    let permissions = sqlx::query_as!(
        GroupPermission,
        "SELECT * FROM group_permissions WHERE group_id = $1",
        group_id
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch group permissions by group_id")?;

    Ok(permissions)
}

pub async fn get_group_permissions_by_permission_id(
    mut transaction: Transaction<'_, Postgres>,
    permission_id: Uuid,
) -> Result<Vec<GroupPermission>, anyhow::Error> {
    let permissions = sqlx::query_as!(
        GroupPermission,
        "SELECT * FROM group_permissions WHERE permission_id = $1",
        permission_id
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch group permissions by permission_id")?;

    Ok(permissions)
}

pub async fn insert_group_permission(
    mut transaction: Transaction<'_, Postgres>,
    group_permission: &GroupPermission,
) -> Result<GroupPermission, anyhow::Error> {
    match sqlx::query_as!(
        GroupPermission,
        "INSERT INTO group_permissions (group_id, permission_id) VALUES ($1, $2) RETURNING group_id, permission_id, created_at",
        group_permission.group_id,
        group_permission.permission_id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to insert group_permission")
    {
        Ok(row) => {
            transaction
                .commit()
                .await
                .context("Failed to commit SQL transaction to insert a new group_permission.")?;
            Ok(row)
        }
        Err(e) => Err(e),
    }
}

pub async fn delete_group_permission(
    mut transaction: Transaction<'_, Postgres>,
    group_id: Uuid,
    permission_id: Uuid,
) -> Result<GroupPermission, anyhow::Error> {
    match sqlx::query_as!(
        GroupPermission,
        "DELETE FROM group_permissions WHERE group_id = $1 AND permission_id = $2 RETURNING group_id, permission_id, created_at",
        group_id,
        permission_id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to delete group_permission")
    {
        Ok(row) => {
            transaction
                .commit()
                .await
                .context("Failed to commit SQL transaction to delete a group_permission.")?;
            Ok(row)
        }
        Err(e) => {
            error!("Failed to delete group_permission: {}", e);
            Err(e)
        }
    }
}

pub async fn delete_group_permissions_by_group_id(
    mut transaction: Transaction<'_, Postgres>,
    group_id: Uuid,
) -> Result<Vec<GroupPermission>, anyhow::Error> {
    let permissions = sqlx::query_as!(
        GroupPermission,
        "DELETE FROM group_permissions WHERE group_id = $1 RETURNING group_id, permission_id, created_at",
        group_id
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to delete group permissions by group_id")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to delete group permissions.")?;

    Ok(permissions)
}

pub async fn delete_group_permissions_by_permission_id(
    mut transaction: Transaction<'_, Postgres>,
    permission_id: Uuid,
) -> Result<Vec<GroupPermission>, anyhow::Error> {
    let permissions = sqlx::query_as!(
        GroupPermission,
        "DELETE FROM group_permissions WHERE permission_id = $1 RETURNING group_id, permission_id, created_at",
        permission_id
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to delete group permissions by permission_id")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to delete group permissions.")?;

    Ok(permissions)
}
