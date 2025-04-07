use crate::dto::query::DtoQuery;
use crate::dto::response::DtoResponse;
use crate::error::AlohaError;
use crate::mappers::tweet::{
    delete_tweet_by_id, get_all_tweets, get_tweet_by_id, get_tweets_by_user_id, insert_tweet,
    update_tweet,
};
use crate::models::tweet::{Tweet, TweetResponse};
use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub(crate) struct CreateTweetFormData {
    content: String,
    user_id: Uuid,
}

/// Create a new tweet
///
/// # API Documentation
///
/// ## POST /api/tweets
///
/// Creates a new tweet with the provided information.
///
/// ### Request Body
/// ```json
/// {
///     "content": "string",
///     "user_id": "uuid"
/// }
/// ```
///
/// ### Response
/// - 200 OK: Returns the created tweet
/// ```json
/// {
///     "id": "integer",
///     "content": "string",
///     "created_at": "datetime",
///     "updated_at": "datetime",
///     "user_id": "uuid"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn insert_tweet_route(
    body: Json<CreateTweetFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    let tweet = Tweet::new(body.content.clone(), body.user_id);
    match insert_tweet(transaction, &tweet).await {
        Ok(result) => Ok(HttpResponse::Ok().json(TweetResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get all tweets with pagination
///
/// # API Documentation
///
/// ## GET /api/tweets
///
/// Retrieves all tweets with optional pagination and filtering.
///
/// ### Query Parameters
/// - page: Page number (optional)
/// - size: Items per page (optional)
/// - sort: Sort field (optional)
/// - order: Sort order (asc/desc) (optional)
///
/// ### Response
/// - 200 OK: Returns list of tweets
/// ```json
/// [
///     {
///         "id": "integer",
///         "content": "string",
///         "created_at": "datetime",
///         "updated_at": "datetime",
///         "user_id": "uuid"
///     }
/// ]
/// ```
/// - 500 Internal Server Error: Database error
pub async fn get_all_tweets_route(
    query: web::Query<DtoQuery>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_all_tweets(transaction, query.0).await {
        Ok(tweets) => {
            let tweet_responses: Vec<TweetResponse> =
                tweets.data.into_iter().map(TweetResponse::from).collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(tweet_responses, tweets.pagination)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get a specific tweet by ID
///
/// # API Documentation
///
/// ## GET /api/tweets/{id}
///
/// Retrieves a specific tweet by its ID.
///
/// ### Path Parameters
/// - id: ID of the tweet
///
/// ### Response
/// - 200 OK: Returns the tweet
/// ```json
/// {
///     "id": "integer",
///     "content": "string",
///     "created_at": "datetime",
///     "updated_at": "datetime",
///     "user_id": "uuid"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn get_tweet_route(
    id: web::Path<(i32,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let tweet_id = id.0;
    let transaction = pool.begin().await.unwrap();
    match get_tweet_by_id(transaction, tweet_id).await {
        Ok(result) => Ok(HttpResponse::Ok().json(TweetResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get tweets by user ID
///
/// # API Documentation
///
/// ## GET /api/users/{user_id}/tweets
///
/// Retrieves all tweets for a specific user.
///
/// ### Path Parameters
/// - user_id: UUID of the user
///
/// ### Query Parameters
/// - page: Page number (optional)
/// - size: Items per page (optional)
/// - sort: Sort field (optional)
/// - order: Sort order (asc/desc) (optional)
///
/// ### Response
/// - 200 OK: Returns list of tweets
/// ```json
/// [
///     {
///         "id": "integer",
///         "content": "string",
///         "created_at": "datetime",
///         "updated_at": "datetime",
///         "user_id": "uuid"
///     }
/// ]
/// ```
/// - 500 Internal Server Error: Database error
pub async fn get_tweets_by_user_route(
    path: web::Path<(Uuid,)>,
    query: web::Query<DtoQuery>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let user_id = path.0;
    let transaction = pool.begin().await.unwrap();
    match get_tweets_by_user_id(transaction, user_id, query.0).await {
        Ok(tweets) => {
            let tweet_responses: Vec<TweetResponse> =
                tweets.data.into_iter().map(TweetResponse::from).collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(tweet_responses, tweets.pagination)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[derive(Deserialize, Clone)]
pub(crate) struct UpdateTweetFormData {
    content: String,
}

/// Update an existing tweet
///
/// # API Documentation
///
/// ## PUT /api/tweets/{id}
///
/// Updates an existing tweet.
///
/// ### Path Parameters
/// - id: ID of the tweet to update
///
/// ### Request Body
/// ```json
/// {
///     "content": "string"
/// }
/// ```
///
/// ### Response
/// - 200 OK: Returns the updated tweet
/// ```json
/// {
///     "id": "integer",
///     "content": "string",
///     "created_at": "datetime",
///     "updated_at": "datetime",
///     "user_id": "uuid"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn update_tweet_route(
    id: web::Path<(i32,)>,
    body: Json<UpdateTweetFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let tweet_id = id.0;
    let transaction = pool.begin().await.unwrap();

    // First get the existing tweet
    let existing_tweet = match get_tweet_by_id(transaction, tweet_id).await {
        Ok(tweet) => tweet,
        Err(e) => return Err(AlohaError::DatabaseError(e.to_string())),
    };

    // Start a new transaction
    let transaction = pool.begin().await.unwrap();

    // Update the tweet with new content
    let updated_tweet = Tweet {
        id: tweet_id,
        content: body.content.clone(),
        created_at: existing_tweet.created_at,
        updated_at: Some(chrono::Utc::now().to_rfc3339()),
        user_id: existing_tweet.user_id,
    };

    match update_tweet(transaction, &updated_tweet).await {
        Ok(result) => Ok(HttpResponse::Ok().json(TweetResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Delete a tweet by ID
///
/// # API Documentation
///
/// ## DELETE /api/tweets/{id}
///
/// Deletes a tweet by its ID.
///
/// ### Path Parameters
/// - id: ID of the tweet to delete
///
/// ### Response
/// - 200 OK: Returns the deleted tweet
/// ```json
/// {
///     "id": "integer",
///     "content": "string",
///     "created_at": "datetime",
///     "updated_at": "datetime",
///     "user_id": "uuid"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn delete_tweet_route(
    id: web::Path<(i32,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let tweet_id = id.0;
    let transaction = pool.begin().await.unwrap();
    match delete_tweet_by_id(transaction, tweet_id).await {
        Ok(result) => Ok(HttpResponse::Ok().json(TweetResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}
