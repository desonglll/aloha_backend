use crate::dto::pagination::Pagination;
use crate::dto::query::DtoQuery;
use crate::dto::response::DtoResponse;
use crate::models::user::User;
use anyhow::Context;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_all_users(
    mut transaction: Transaction<'_, Postgres>,
    dto_query: DtoQuery,
) -> Result<DtoResponse<Vec<User>>, anyhow::Error> {
    let offset = dto_query.offset() as i64;
    let limit = dto_query.size() as i64;
    let total = sqlx::query!("SELECT COUNT(*) FROM users")
        .fetch_one(&mut *transaction)
        .await?
        .count;

    let rows = sqlx::query!(
        r#"
        SELECT id, username, password_hash, created_at, user_group_id 
        FROM users 
        ORDER BY id 
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch paginated users")?;

    let data = rows
        .into_iter()
        .map(|row| User {
            id: row.id,
            username: row.username,
            password_hash: row.password_hash,
            created_at: row.created_at.map(|dt| dt.to_string()),
            user_group_id: row.user_group_id,
        })
        .collect();

    let pagination = Pagination::new(
        Option::from(dto_query.page()),
        Option::from(dto_query.size()),
        total,
    );
    Ok(DtoResponse::new(data, Option::from(pagination)))
}

pub async fn get_user_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: Uuid,
) -> Result<User, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, username, password_hash, created_at, user_group_id 
        FROM users 
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to fetch user")?;

    Ok(User {
        id: row.id,
        username: row.username,
        password_hash: row.password_hash,
        created_at: row.created_at.map(|dt| dt.to_string()),
        user_group_id: row.user_group_id,
    })
}

pub async fn get_user_by_username(
    mut transaction: Transaction<'_, Postgres>,
    username: &str,
) -> Result<User, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, username, password_hash, created_at, user_group_id 
        FROM users 
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to fetch user by username")?;

    Ok(User {
        id: row.id,
        username: row.username,
        password_hash: row.password_hash,
        created_at: row.created_at.map(|dt| dt.to_string()),
        user_group_id: row.user_group_id,
    })
}

pub async fn insert_user(
    mut transaction: Transaction<'_, Postgres>,
    user: &User,
) -> Result<User, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO users (id, username, password_hash, user_group_id) 
        VALUES ($1, $2, $3, $4) 
        RETURNING id, username, password_hash, created_at, user_group_id
        "#,
        user.id,
        user.username,
        user.password_hash,
        user.user_group_id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to insert user")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to insert a new user.")?;

    Ok(User {
        id: row.id,
        username: row.username,
        password_hash: row.password_hash,
        created_at: row.created_at.map(|dt| dt.to_string()),
        user_group_id: row.user_group_id,
    })
}

pub async fn delete_user_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: Uuid,
) -> Result<User, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        DELETE FROM users 
        WHERE id = $1 
        RETURNING id, username, password_hash, created_at, user_group_id
        "#,
        id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to delete user")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to delete a user.")?;

    Ok(User {
        id: row.id,
        username: row.username,
        password_hash: row.password_hash,
        created_at: row.created_at.map(|dt| dt.to_string()),
        user_group_id: row.user_group_id,
    })
}

pub async fn update_user(
    mut transaction: Transaction<'_, Postgres>,
    user: &User,
) -> Result<User, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        UPDATE users 
        SET username = $1, password_hash = $2, user_group_id = $3 
        WHERE id = $4 
        RETURNING id, username, password_hash, created_at, user_group_id
        "#,
        user.username,
        user.password_hash,
        user.user_group_id,
        user.id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to update user")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to update a user.")?;

    Ok(User {
        id: row.id,
        username: row.username,
        password_hash: row.password_hash,
        created_at: row.created_at.map(|dt| dt.to_string()),
        user_group_id: row.user_group_id,
    })
}

pub async fn delete_users_by_ids(
    mut transaction: Transaction<'_, Postgres>,
    ids: Vec<Uuid>,
) -> Result<Vec<User>, anyhow::Error> {
    let rows = sqlx::query!(
        r#"
        DELETE FROM users 
        WHERE id = ANY($1) 
        RETURNING id, username, password_hash, created_at, user_group_id
        "#,
        &ids
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to delete users")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to delete users.")?;

    let users = rows
        .into_iter()
        .map(|row| User {
            id: row.id,
            username: row.username,
            password_hash: row.password_hash,
            created_at: row.created_at.map(|dt| dt.to_string()),
            user_group_id: row.user_group_id,
        })
        .collect();

    Ok(users)
}
