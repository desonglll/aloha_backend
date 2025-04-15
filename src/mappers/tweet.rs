use crate::dto::query::DtoQuery;
use crate::dto::response::DtoResponse;
use crate::dto::{pagination::Pagination, query::TweetFilterQuery};
use crate::models::tweet::Tweet;
use anyhow::Context;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_all_tweets(
    mut transaction: Transaction<'_, Postgres>,
    dto_query: DtoQuery<TweetFilterQuery>,
) -> Result<DtoResponse<Vec<Tweet>>, anyhow::Error> {
    let offset = dto_query.offset() as i64;
    let limit = dto_query.size() as i64;
    let total = sqlx::query!("SELECT COUNT(*) FROM tweet")
        .fetch_one(&mut *transaction)
        .await?
        .count;

    let user_id = dto_query.filter.as_ref().and_then(|f| f.user_id);

    let rows = sqlx::query!(
        r#"
        SELECT id, content, created_at, updated_at, user_id 
        FROM tweet 
        WHERE ($1::uuid IS NULL OR user_id = $1)
        ORDER BY created_at DESC 
        LIMIT $2 OFFSET $3
        "#,
        user_id,
        limit,
        offset
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to fetch paginated tweets")?;

    let data: Vec<Tweet> = rows
        .into_iter()
        .map(|row| Tweet {
            id: row.id,
            content: row.content,
            created_at: row.created_at,
            updated_at: row.updated_at,
            user_id: row.user_id,
        })
        .collect();

    let pagination = Pagination::new(Some(dto_query.page()), Some(dto_query.size()), total);
    Ok(DtoResponse::new(data, Some(pagination)))
}

pub async fn get_tweet_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: Uuid,
) -> Result<Option<Tweet>, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, content, created_at, updated_at, user_id 
        FROM tweet 
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&mut *transaction)
    .await
    .context("Failed to fetch tweet by id")?;

    Ok(row.map(|row| Tweet {
        id: row.id,
        content: row.content,
        created_at: row.created_at,
        updated_at: row.updated_at,
        user_id: row.user_id,
    }))
}

pub async fn insert_tweet(
    mut transaction: Transaction<'_, Postgres>,
    tweet: &Tweet,
) -> Result<Tweet, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO tweet (id, content, user_id)
        VALUES ($1, $2, $3)
        RETURNING id, content, created_at, updated_at, user_id
        "#,
        tweet.id,
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
        content: row.content,
        created_at: row.created_at,
        updated_at: row.updated_at,
        user_id: row.user_id,
    })
}

pub async fn delete_tweet_by_id(
    mut transaction: Transaction<'_, Postgres>,
    id: Uuid,
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
        content: row.content,
        created_at: row.created_at,
        updated_at: row.updated_at,
        user_id: row.user_id,
    })
}

pub async fn update_tweet(
    mut transaction: Transaction<'_, Postgres>,
    tweet: &Tweet,
) -> Result<Tweet, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        UPDATE tweet
        SET content = $1
        WHERE id = $2
        RETURNING id, content, created_at, updated_at, user_id
        "#,
        tweet.content,
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
        content: row.content,
        created_at: row.created_at,
        updated_at: row.updated_at,
        user_id: row.user_id,
    })
}

pub async fn delete_tweets_by_ids(
    mut transaction: Transaction<'_, Postgres>,
    ids: Vec<Uuid>,
) -> Result<Vec<Tweet>, anyhow::Error> {
    let rows = sqlx::query!(
        r#"
        DELETE FROM tweet
        WHERE id = ANY($1)
        RETURNING id, content, created_at, updated_at, user_id
        "#,
        &ids as &[Uuid]
    )
    .fetch_all(&mut *transaction)
    .await
    .context("Failed to delete tweets")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to delete tweets.")?;

    let tweets = rows
        .into_iter()
        .map(|row| Tweet {
            id: row.id,
            content: row.content,
            created_at: row.created_at,
            updated_at: row.updated_at,
            user_id: row.user_id,
        })
        .collect();

    Ok(tweets)
}
