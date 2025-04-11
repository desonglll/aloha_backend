use crate::dto::pagination::Pagination;
use crate::dto::query::{DtoQuery, PermissionFilterQuery};
use crate::dto::response::DtoResponse;
use anyhow::{Context, Result};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::models::permission::Permission;

pub async fn get_all_permissions(
    mut transaction: Transaction<'_, Postgres>,
    dto_query: DtoQuery<PermissionFilterQuery>,
) -> Result<DtoResponse<Vec<Permission>>, anyhow::Error> {
    let offset = dto_query.offset() as i64;
    let limit = dto_query.size() as i64;
    let total = sqlx::query!("SELECT COUNT(*) FROM permissions")
        .fetch_one(&mut *transaction)
        .await?
        .count;

    let permissions = sqlx::query_as!(
        Permission,
        r#"
        SELECT id, name, description, created_at
        FROM permissions
        ORDER BY id
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch paginated permissions")?;

    let pagination = Pagination::new(
        Option::from(dto_query.page()),
        Option::from(dto_query.size()),
        total,
    );
    Ok(DtoResponse::new(permissions, Option::from(pagination)))
}

pub async fn get_permission_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: Uuid,
) -> Result<Option<Permission>> {
    let permission = sqlx::query_as!(
        Permission,
        r#"
        SELECT id, name, description, created_at
        FROM permissions
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&mut *transaction)
    .await
    .context("Failed to fetch permission by id")?;

    Ok(permission)
}

pub async fn get_permission_by_name(
    mut transaction: Transaction<'_, Postgres>,
    name: &str,
) -> Result<Option<Permission>> {
    let permission = sqlx::query_as!(
        Permission,
        r#"
        SELECT id, name, description, created_at
        FROM permissions
        WHERE name = $1
        "#,
        name
    )
    .fetch_optional(&mut *transaction)
    .await
    .context("Failed to fetch permission by name")?;

    Ok(permission)
}

pub async fn insert_permission(
    mut transaction: Transaction<'_, Postgres>,
    permission: &Permission,
) -> Result<Permission, anyhow::Error> {
    let permission = sqlx::query_as!(
        Permission,
        r#"
        INSERT INTO permissions (id, name, description, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, description, created_at
        "#,
        permission.id,
        permission.name,
        permission.description,
        permission.created_at
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to insert permission")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to insert a new permission.")?;
    Ok(permission)
}

pub async fn delete_permission_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: Uuid,
) -> Result<Permission, anyhow::Error> {
    let permission = sqlx::query_as!(
        Permission,
        r#"
        DELETE FROM permissions
        WHERE id = $1
        RETURNING id, name, description, created_at
        "#,
        id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to delete permission")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to delete a permission.")?;
    Ok(permission)
}

pub async fn update_permission(
    mut transaction: Transaction<'_, Postgres>,
    permission: &Permission,
) -> Result<Permission, anyhow::Error> {
    let permission = sqlx::query_as!(
        Permission,
        r#"
        UPDATE permissions
        SET name = $1, description = $2
        WHERE id = $3
        RETURNING id, name, description, created_at
        "#,
        permission.name,
        permission.description,
        permission.id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to update permission")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to update a permission.")?;
    Ok(permission)
}
