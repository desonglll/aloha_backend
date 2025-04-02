use crate::mappers::user_group::{delete_user_group_by_id, get_all_groups, get_group_by_id, insert_user_group, update_user_group};
use crate::models::user_group::UserGroup;
use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub(crate) struct FormData {
    group_name: String,
}

pub async fn insert_user_group_route(
    body: Json<FormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let group_name = body.group_name.clone();
    let transaction = pool.begin().await.unwrap();
    let user_group = UserGroup::new(group_name.clone());
    let result = insert_user_group(transaction, &user_group).await.unwrap();
    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_all_user_groups_route(pool: Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let transaction = pool.begin().await.unwrap();
    let user_groups = get_all_groups(transaction).await.unwrap();
    Ok(HttpResponse::Ok().json(user_groups))
}

pub async fn get_user_group_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = id.0;
    let transaction = pool.begin().await.unwrap();
    let result = get_group_by_id(transaction, user_id).await.unwrap();
    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_user_group_route(
    body: Json<UserGroup>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let transaction = pool.begin().await.unwrap();
    let result = update_user_group(transaction, &body).await.unwrap();
    Ok(HttpResponse::Ok().json(result))
}

pub async fn delete_user_group_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let transaction = pool.begin().await.unwrap();
    let result = delete_user_group_by_id(transaction, id.0).await.unwrap();
    Ok(HttpResponse::Ok().json(result))
}
