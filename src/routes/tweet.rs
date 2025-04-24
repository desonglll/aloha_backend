use crate::configuration::get_configuration;
use crate::dto::query::{DtoQuery, TweetFilterQuery};
use crate::dto::response::DtoResponse;
use crate::error::AlohaError;
use crate::mappers::tweet::{
    delete_tweet_by_id, delete_tweets_by_ids, get_all_tweets, get_tweet_by_id, insert_tweet,
    update_tweet,
};
use crate::models::tweet::{Tweet, TweetResponse};
use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_qs::actix::QsQuery;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Clone, ToSchema)]
pub struct CreateTweetFormData {
    content: String,
    user_id: Uuid,
}

#[utoipa::path(
    post,
    path = "/api/tweets",
    request_body = CreateTweetFormData,
    responses(
        (status = 200, description = "Tweet created successfully", body = TweetResponse),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
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

#[utoipa::path(
    get,
    path = "/api/tweets",
    params(
        ("page" = Option<i32>, Query, description = "Page number"),
        ("size" = Option<i32>, Query, description = "Page size"),
        ("user_id" = Option<Uuid>, Query, description = "Filter by user ID")
    ),
    responses(
        (status = 200, description = "Tweets retrieved successfully", body = DtoResponse<Vec<TweetResponse>>),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_all_tweets_route(
    query: QsQuery<DtoQuery<TweetFilterQuery>>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_all_tweets(transaction, query.into_inner()).await {
        Ok(result) => {
            let response: Vec<TweetResponse> =
                result.data.into_iter().map(TweetResponse::from).collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(response, result.pagination)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/tweets/{id}",
    params(
        ("id" = Uuid, Path, description = "Tweet ID")
    ),
    responses(
        (status = 200, description = "Tweet retrieved successfully", body = TweetResponse),
        (status = 404, description = "Tweet not found"),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_tweet_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_tweet_by_id(transaction, id.0).await {
        Ok(Some(result)) => Ok(HttpResponse::Ok().json(TweetResponse::from(result))),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[derive(Deserialize, Clone, ToSchema)]
pub struct PutTweetFormData {
    pub id: Uuid,
    pub content: String,
}

#[utoipa::path(
    put,
    path = "/api/tweets/{id}",
    request_body = PutTweetFormData,
    responses(
        (status = 200, description = "Tweet updated successfully", body = TweetResponse),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn update_tweet_route(
    body: Json<PutTweetFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    let tweet = Tweet {
        id: body.id,
        content: body.content.clone(),
        created_at: None,
        updated_at: None,
        user_id: Uuid::nil(), // This will be ignored in the update query
    };
    match update_tweet(transaction, &tweet).await {
        Ok(result) => Ok(HttpResponse::Ok().json(TweetResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/api/tweets",
    request_body = Vec<Uuid>,
    responses(
        (status = 200, description = "Tweets deleted successfully", body = Vec<TweetResponse>),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_tweets_route(
    body: Json<Vec<Uuid>>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_tweets_by_ids(transaction, body.0).await {
        Ok(result) => Ok(HttpResponse::Ok().json(
            result
                .into_iter()
                .map(TweetResponse::from)
                .collect::<Vec<TweetResponse>>(),
        )),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/api/tweets/{id}",
    params(
        ("id" = Uuid, Path, description = "Tweet ID")
    ),
    responses(
        (status = 200, description = "Tweet deleted successfully", body = TweetResponse),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_tweet_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_tweet_by_id(transaction, id.0).await {
        Ok(result) => Ok(HttpResponse::Ok().json(TweetResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

pub fn tweet_routes(cfg: &mut web::ServiceConfig) {
    let config = get_configuration().unwrap();

    cfg.service(
        web::scope(format!("/{}", config.routes.tweets).as_str())
            .route("", web::post().to(insert_tweet_route))
            .route("", web::get().to(get_all_tweets_route))
            .route("/{id}", web::get().to(get_tweet_route))
            .route("/{id}", web::put().to(update_tweet_route))
            .route("", web::delete().to(delete_tweets_route))
            .route("/{id}", web::delete().to(delete_tweet_route)),
    );
}
