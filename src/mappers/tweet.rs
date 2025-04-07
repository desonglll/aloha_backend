use crate::dto::pagination::Pagination;
use crate::dto::query::DtoQuery;
use crate::dto::response::DtoResponse;
use crate::models::tweet::Tweet;
use anyhow::Context;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_all_tweets(
    mut transaction: Transaction<'_, Postgres>,
    dto_query: DtoQuery,
) -> Result<DtoResponse<Vec<Tweet>>, anyhow::Error> {
    let offset = dto_query.offset() as i64;
    let limit = dto_query.size() as i64;
    let total = sqlx::query!("SELECT COUNT(*) FROM tweet")
        .fetch_one(&mut *transaction)
        .await?
        .count;

    let rows = sqlx::query!(
        r#"
        SELECT id, content, created_at, updated_at, user_id 
        FROM tweet 
        ORDER BY id 
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch paginated tweets")?;

    let data = rows
        .into_iter()
        .map(|row| Tweet {
            id: row.id,
            content: row.content.unwrap_or_default(),
            created_at: row.created_at.map(|dt| dt.to_string()),
            updated_at: row.updated_at.map(|dt| dt.to_string()),
            user_id: row.user_id.unwrap(),
        })
        .collect();

    let pagination = Pagination::new(
        Option::from(dto_query.page()),
        Option::from(dto_query.size()),
        total,
    );
    Ok(DtoResponse::new(data, Option::from(pagination)))
}

pub async fn get_tweet_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: i32,
) -> Result<Tweet, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, content, created_at, updated_at, user_id 
        FROM tweet 
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to fetch tweet")?;

    Ok(Tweet {
        id: row.id,
        content: row.content.unwrap_or_default(),
        created_at: row.created_at.map(|dt| dt.to_string()),
        updated_at: row.updated_at.map(|dt| dt.to_string()),
        user_id: row.user_id.unwrap(),
    })
}

pub async fn get_tweets_by_user_id(
    mut transaction: Transaction<'_, Postgres>,
    user_id: Uuid,
    dto_query: DtoQuery,
) -> Result<DtoResponse<Vec<Tweet>>, anyhow::Error> {
    let offset = dto_query.offset() as i64;
    let limit = dto_query.size() as i64;
    let total = sqlx::query!("SELECT COUNT(*) FROM tweet WHERE user_id = $1", user_id)
        .fetch_one(&mut *transaction)
        .await?
        .count;

    let rows = sqlx::query!(
        r#"
        SELECT id, content, created_at, updated_at, user_id 
        FROM tweet 
        WHERE user_id = $1 
        ORDER BY id 
        LIMIT $2 OFFSET $3
        "#,
        user_id,
        limit,
        offset
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch tweets for user")?;

    let data = rows
        .into_iter()
        .map(|row| Tweet {
            id: row.id,
            content: row.content.unwrap_or_default(),
            created_at: row.created_at.map(|dt| dt.to_string()),
            updated_at: row.updated_at.map(|dt| dt.to_string()),
            user_id: row.user_id.unwrap(),
        })
        .collect();

    let pagination = Pagination::new(
        Option::from(dto_query.page()),
        Option::from(dto_query.size()),
        total,
    );
    Ok(DtoResponse::new(data, Option::from(pagination)))
}

pub async fn insert_tweet(
    mut transaction: Transaction<'_, Postgres>,
    tweet: &Tweet,
) -> Result<Tweet, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO tweet (content, user_id) 
        VALUES ($1, $2) 
        RETURNING id, content, created_at, updated_at, user_id
        "#,
        tweet.content,
        tweet.user_id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to insert tweet")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to insert a new tweet.")?;

    Ok(Tweet {
        id: row.id,
        content: row.content.unwrap_or_default(),
        created_at: row.created_at.map(|dt| dt.to_string()),
        updated_at: row.updated_at.map(|dt| dt.to_string()),
        user_id: row.user_id.unwrap(),
    })
}

pub async fn delete_tweet_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: i32,
) -> Result<Tweet, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        DELETE FROM tweet 
        WHERE id = $1 
        RETURNING id, content, created_at, updated_at, user_id
        "#,
        id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to delete tweet")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to delete a tweet.")?;

    Ok(Tweet {
        id: row.id,
        content: row.content.unwrap_or_default(),
        created_at: row.created_at.map(|dt| dt.to_string()),
        updated_at: row.updated_at.map(|dt| dt.to_string()),
        user_id: row.user_id.unwrap(),
    })
}

pub async fn update_tweet(
    mut transaction: Transaction<'_, Postgres>,
    tweet: &Tweet,
) -> Result<Tweet, anyhow::Error> {
    let now = chrono::Utc::now();

    let row = sqlx::query!(
        r#"
        UPDATE tweet 
        SET content = $1, updated_at = $2
        WHERE id = $3 
        RETURNING id, content, created_at, updated_at, user_id
        "#,
        tweet.content,
        now,
        tweet.id
    )
    .fetch_one(&mut *transaction)
    .await
    .context("Failed to update tweet")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to update a tweet.")?;

    Ok(Tweet {
        id: row.id,
        content: row.content.unwrap_or_default(),
        created_at: row.created_at.map(|dt| dt.to_string()),
        updated_at: row.updated_at.map(|dt| dt.to_string()),
        user_id: row.user_id.unwrap(),
    })
}
