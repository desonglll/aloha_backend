use crate::configuration::get_configuration;
use crate::dto::query::{DtoQuery, UserGroupFilterQuery};
use crate::dto::response::DtoResponse;
use crate::error::AlohaError;
use crate::mappers::user_group::{
    delete_user_group_by_id, get_all_groups, get_group_by_id, insert_user_group, update_user_group,
};
use crate::models::user_group::{UserGroup, UserGroupResponse};
use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Clone, ToSchema)]
pub struct CreateUserGroupFormData {
    pub group_name: String,
}

#[utoipa::path(
    post,
    path = "/api/user_groups",
    request_body = CreateUserGroupFormData,
    responses(
        (status = 200, description = "User group created successfully", body = UserGroup),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn insert_user_group_route(
    body: Json<CreateUserGroupFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    let user_group = UserGroup::from(body.0);
    match insert_user_group(transaction, &user_group).await {
        Ok(result) => Ok(HttpResponse::Ok().json(UserGroupResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/user_groups",
    params(
        ("page" = Option<i32>, Query, description = "Page number"),
        ("size" = Option<i32>, Query, description = "Page size"),
        ("sort" = Option<String>, Query, description = "Sort field"),
        ("order" = Option<String>, Query, description = "Sort order (asc/desc)")
    ),
    responses(
        (status = 200, description = "User groups retrieved successfully", body = DtoResponse<Vec<UserGroup>>),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_all_user_groups_route(
    query: web::Query<DtoQuery<UserGroupFilterQuery>>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_all_groups(transaction, query.0).await {
        Ok(user_groups) => {
            let groups: Vec<UserGroupResponse> = user_groups
                .data
                .into_iter()
                .map(UserGroupResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(groups, user_groups.pagination)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/user_groups/{id}",
    params(
        ("id" = Uuid, Path, description = "User group ID")
    ),
    responses(
        (status = 200, description = "User group retrieved successfully", body = UserGroup),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_user_group_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let user_id = id.0;
    let transaction = pool.begin().await.unwrap();
    match get_group_by_id(transaction, user_id).await {
        Ok(result) => Ok(HttpResponse::Ok().json(UserGroupResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[derive(Deserialize, Serialize, Clone, ToSchema)]
pub struct PutUserGroupFormData {
    pub id: Uuid,
    pub group_name: String,
}

#[utoipa::path(
    put,
    path = "/api/user_groups",
    request_body = PutUserGroupFormData,
    responses(
        (status = 200, description = "User group updated successfully", body = UserGroup),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn update_user_group_route(
    body: Json<PutUserGroupFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();

    let mut user_group = get_group_by_id(transaction, body.0.id).await.unwrap();
    user_group.group_name = body.group_name.clone();

    let transaction = pool.begin().await.unwrap();

    match update_user_group(transaction, &user_group).await {
        Ok(result) => Ok(HttpResponse::Ok().json(UserGroupResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/api/user_groups/{id}",
    params(
        ("id" = Uuid, Path, description = "User group ID")
    ),
    responses(
        (status = 200, description = "User group deleted successfully", body = UserGroup),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_user_group_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_user_group_by_id(transaction, id.0).await {
        Ok(result) => Ok(HttpResponse::Ok().json(UserGroupResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}
pub fn user_group_routes(cfg: &mut web::ServiceConfig) {
    let config = get_configuration().unwrap();
    cfg.service(
        web::scope(format!("/{}", config.routes.user_groups).as_str())
            .route("", web::post().to(insert_user_group_route))
            .route("/{id}", web::get().to(get_user_group_route))
            .route("", web::put().to(update_user_group_route))
            .route("", web::get().to(get_all_user_groups_route))
            .route("/{id}", web::delete().to(delete_user_group_route)),
    );
}
