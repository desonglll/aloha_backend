use crate::dto::query::DtoQuery;
use crate::dto::response::DtoResponse;
use crate::dto::{pagination::Pagination, query::UserPermissionFilterQuery};
use crate::models::user_permission::UserPermission;
use anyhow::{Context, Result};
use sqlx::{Postgres, Transaction};
use tracing::error;
use uuid::Uuid;

pub async fn get_all_user_permissions(
    mut transaction: Transaction<'_, Postgres>,
    dto_query: DtoQuery<UserPermissionFilterQuery>,
) -> Result<DtoResponse<Vec<UserPermission>>, anyhow::Error> {
    let offset = dto_query.offset() as i64;
    let limit = dto_query.size() as i64;
    let total = sqlx::query!("SELECT COUNT(*) FROM user_permissions")
        .fetch_one(&mut *transaction)
        .await?
        .count;

    let data = sqlx::query_as!(
        UserPermission,
        "SELECT * FROM user_permissions ORDER BY user_id, permission_id LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch paginated user_permissions")?;

    let pagination = Pagination::new(
        Option::from(dto_query.page()),
        Option::from(dto_query.size()),
        total,
    );
    Ok(DtoResponse::new(data, Option::from(pagination)))
}

pub async fn get_user_permissions_by_user_id(
    mut transaction: Transaction<'_, Postgres>,
    user_id: Uuid,
) -> Result<Vec<UserPermission>, anyhow::Error> {
    let permissions = sqlx::query_as!(
        UserPermission,
        "SELECT * FROM user_permissions WHERE user_id = $1",
        user_id
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch user permissions by user_id")?;

    Ok(permissions)
}

pub async fn get_user_permissions_by_permission_id(
    mut transaction: Transaction<'_, Postgres>,
    permission_id: Uuid,
) -> Result<Vec<UserPermission>, anyhow::Error> {
    let permissions = sqlx::query_as!(
        UserPermission,
        "SELECT * FROM user_permissions WHERE permission_id = $1",
        permission_id
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch user permissions by permission_id")?;

    Ok(permissions)
}

pub async fn insert_user_permission(
    mut transaction: Transaction<'_, Postgres>,
    user_permission: &UserPermission,
) -> Result<UserPermission, anyhow::Error> {
    match sqlx::query_as!(
        UserPermission,
        "INSERT INTO user_permissions (user_id, permission_id) VALUES ($1, $2) RETURNING user_id, permission_id, created_at",
        user_permission.user_id,
        user_permission.permission_id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to insert user_permission")
    {
        Ok(row) => {
            transaction
                .commit()
                .await
                .context("Failed to commit SQL transaction to insert a new user_permission.")?;
            Ok(row)
        }
        Err(e) => Err(e),
    }
}

pub async fn delete_user_permission(
    mut transaction: Transaction<'_, Postgres>,
    user_id: Uuid,
    permission_id: Uuid,
) -> Result<UserPermission, anyhow::Error> {
    match sqlx::query_as!(
        UserPermission,
        "DELETE FROM user_permissions WHERE user_id = $1 AND permission_id = $2 RETURNING user_id, permission_id, created_at",
        user_id,
        permission_id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to delete user_permission")
    {
        Ok(row) => {
            transaction
                .commit()
                .await
                .context("Failed to commit SQL transaction to delete a user_permission.")?;
            Ok(row)
        }
        Err(e) => {
            error!("Failed to delete user_permission: {}", e);
            Err(e)
        }
    }
}

pub async fn delete_user_permissions_by_user_id(
    mut transaction: Transaction<'_, Postgres>,
    user_id: Uuid,
) -> Result<Vec<UserPermission>, anyhow::Error> {
    let permissions = sqlx::query_as!(
        UserPermission,
        "DELETE FROM user_permissions WHERE user_id = $1 RETURNING user_id, permission_id, created_at",
        user_id
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to delete user permissions by user_id")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to delete user permissions.")?;

    Ok(permissions)
}

pub async fn delete_user_permissions_by_permission_id(
    mut transaction: Transaction<'_, Postgres>,
    permission_id: Uuid,
) -> Result<Vec<UserPermission>, anyhow::Error> {
    let permissions = sqlx::query_as!(
        UserPermission,
        "DELETE FROM user_permissions WHERE permission_id = $1 RETURNING user_id, permission_id, created_at",
        permission_id
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to delete user permissions by permission_id")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to delete user permissions.")?;

    Ok(permissions)
}
