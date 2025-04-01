use crate::mappers::user_group::{get_group_by_id, insert_user_group};
use crate::models::user_group::UserGroup;
use actix_web::web::{Data, Form};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct FormData {
    group_name: String,
}

pub async fn insert_user_group_route(
    form: Form<FormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let group_name = form.group_name.clone();
    let transaction = pool.begin().await.unwrap();
    let user_group = UserGroup::new(group_name.clone());
    let result = insert_user_group(transaction, &user_group).await.unwrap();
    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_user_group_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
)->Result<HttpResponse, actix_web::Error>{
    let user_id = id.0;
    let transaction = pool.begin().await.unwrap();
    let result = get_group_by_id(transaction, user_id).await.unwrap();
    Ok(HttpResponse::Ok().json(result))
}