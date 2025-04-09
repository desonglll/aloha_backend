use crate::configuration::get_configuration;
use crate::dto::query::DtoQuery;
use crate::dto::response::DtoResponse;
use crate::error::AlohaError;
use crate::mappers::user_permission::{
    delete_user_permission, delete_user_permissions_by_permission_id,
    delete_user_permissions_by_user_id, get_all_user_permissions,
    get_user_permissions_by_permission_id, get_user_permissions_by_user_id, insert_user_permission,
};
use crate::models::user_permission::{UserPermission, UserPermissionResponse};
use actix_web::web::{self, Data, Json, Path, Query};
use actix_web::HttpResponse;
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Clone, ToSchema)]
pub struct CreateUserPermissionFormData {
    pub user_id: Uuid,
    pub permission_id: Uuid,
}

#[utoipa::path(
    post,
    path = "/api/user_permissions",
    request_body = CreateUserPermissionFormData,
    responses(
        (status = 200, description = "User permission created successfully", body = UserPermission),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn insert_user_permission_route(
    body: Json<CreateUserPermissionFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    let user_permission = UserPermission::from(body.0);
    match insert_user_permission(transaction, &user_permission).await {
        Ok(result) => Ok(HttpResponse::Ok().json(UserPermissionResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/user_permissions",
    params(
        ("page" = Option<i32>, Query, description = "Page number"),
        ("size" = Option<i32>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "User permissions retrieved successfully", body = DtoResponse<Vec<UserPermission>>),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_all_user_permissions_route(
    query: Query<DtoQuery>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_all_user_permissions(transaction, query.0).await {
        Ok(user_permissions) => {
            let result: Vec<UserPermissionResponse> = user_permissions
                .data
                .into_iter()
                .map(UserPermissionResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(result, user_permissions.pagination)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/user_permissions/user/{user_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User permissions retrieved successfully", body = Vec<UserPermission>),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_user_permissions_by_user_id_route(
    user_id: Path<Uuid>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_user_permissions_by_user_id(transaction, *user_id).await {
        Ok(user_permissions) => {
            let result: Vec<UserPermissionResponse> = user_permissions
                .into_iter()
                .map(UserPermissionResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(result, None)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/user_permissions/permission/{permission_id}",
    params(
        ("permission_id" = Uuid, Path, description = "Permission ID")
    ),
    responses(
        (status = 200, description = "User permissions retrieved successfully", body = Vec<UserPermission>),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_user_permissions_by_permission_id_route(
    permission_id: Path<Uuid>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_user_permissions_by_permission_id(transaction, *permission_id).await {
        Ok(user_permissions) => {
            let result: Vec<UserPermissionResponse> = user_permissions
                .into_iter()
                .map(UserPermissionResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(result, None)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[derive(Deserialize, Clone, ToSchema)]
pub struct DeleteUserPermissionFormData {
    pub user_id: Uuid,
    pub permission_id: Uuid,
}

#[utoipa::path(
    delete,
    path = "/api/user_permissions",
    request_body = DeleteUserPermissionFormData,
    responses(
        (status = 200, description = "User permission deleted successfully", body = UserPermission),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_user_permission_route(
    body: Json<DeleteUserPermissionFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_user_permission(transaction, body.user_id, body.permission_id).await {
        Ok(result) => Ok(HttpResponse::Ok().json(UserPermissionResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/api/user_permissions/user/{user_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User permissions deleted successfully", body = Vec<UserPermission>),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_user_permissions_by_user_id_route(
    user_id: Path<Uuid>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_user_permissions_by_user_id(transaction, *user_id).await {
        Ok(user_permissions) => {
            let result: Vec<UserPermissionResponse> = user_permissions
                .into_iter()
                .map(UserPermissionResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(result, None)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/api/user_permissions/permission/{permission_id}",
    params(
        ("permission_id" = Uuid, Path, description = "Permission ID")
    ),
    responses(
        (status = 200, description = "User permissions deleted successfully", body = Vec<UserPermission>),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_user_permissions_by_permission_id_route(
    permission_id: Path<Uuid>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_user_permissions_by_permission_id(transaction, *permission_id).await {
        Ok(user_permissions) => {
            let result: Vec<UserPermissionResponse> = user_permissions
                .into_iter()
                .map(UserPermissionResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(result, None)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

pub fn user_permissions_routes(cfg: &mut web::ServiceConfig) {
    let config = get_configuration().unwrap();
    cfg.service(
        web::scope(format!("/{}", config.routes.user_permissions).as_str())
            .route("", web::post().to(insert_user_permission_route))
            .route("", web::get().to(get_all_user_permissions_route))
            .route(
                "/user/{user_id}",
                web::get().to(get_user_permissions_by_user_id_route),
            )
            .route(
                "/permission/{permission_id}",
                web::get().to(get_user_permissions_by_permission_id_route),
            )
            .route("", web::delete().to(delete_user_permission_route))
            .route(
                "/user/{user_id}",
                web::delete().to(delete_user_permissions_by_user_id_route),
            )
            .route(
                "/permission/{permission_id}",
                web::delete().to(delete_user_permissions_by_permission_id_route),
            ),
    );
}
