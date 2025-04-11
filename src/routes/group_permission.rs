use crate::configuration::get_configuration;
use crate::dto::query::{DtoQuery, GroupPermissionFilterQuery};
use crate::dto::response::DtoResponse;
use crate::error::AlohaError;
use crate::mappers::group_permission::{
    delete_group_permission, delete_group_permissions_by_group_id,
    delete_group_permissions_by_permission_id, get_all_group_permissions,
    get_group_permissions_by_group_id, get_group_permissions_by_permission_id,
    insert_group_permission,
};
use crate::models::group_permission::{GroupPermission, GroupPermissionResponse};
use actix_web::web::{self, Data, Json, Path, Query};
use actix_web::HttpResponse;
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Clone, ToSchema)]
pub struct CreateGroupPermissionFormData {
    pub group_id: Uuid,
    pub permission_id: Uuid,
}

#[utoipa::path(
    post,
    path = "/api/group_permissions",
    request_body = CreateGroupPermissionFormData,
    responses(
        (status = 200, description = "Group permission created successfully", body = GroupPermission),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn insert_group_permission_route(
    body: Json<CreateGroupPermissionFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    let group_permission = GroupPermission::from(body.0);
    match insert_group_permission(transaction, &group_permission).await {
        Ok(result) => Ok(HttpResponse::Ok().json(GroupPermissionResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/group_permissions",
    params(
        ("page" = Option<i32>, Query, description = "Page number"),
        ("size" = Option<i32>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Group permissions retrieved successfully", body = DtoResponse<Vec<GroupPermission>>),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_all_group_permissions_route(
    query: Query<DtoQuery<GroupPermissionFilterQuery>>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_all_group_permissions(transaction, query.0).await {
        Ok(group_permissions) => {
            let result: Vec<GroupPermissionResponse> = group_permissions
                .data
                .into_iter()
                .map(GroupPermissionResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(result, group_permissions.pagination)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/group_permissions/group/{group_id}",
    params(
        ("group_id" = Uuid, Path, description = "Group ID")
    ),
    responses(
        (status = 200, description = "Group permissions retrieved successfully", body = Vec<GroupPermission>),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_group_permissions_by_group_id_route(
    group_id: Path<Uuid>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_group_permissions_by_group_id(transaction, *group_id).await {
        Ok(group_permissions) => {
            let result: Vec<GroupPermissionResponse> = group_permissions
                .into_iter()
                .map(GroupPermissionResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(result, None)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/group_permissions/permission/{permission_id}",
    params(
        ("permission_id" = Uuid, Path, description = "Permission ID")
    ),
    responses(
        (status = 200, description = "Group permissions retrieved successfully", body = Vec<GroupPermission>),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_group_permissions_by_permission_id_route(
    permission_id: Path<Uuid>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_group_permissions_by_permission_id(transaction, *permission_id).await {
        Ok(group_permissions) => {
            let result: Vec<GroupPermissionResponse> = group_permissions
                .into_iter()
                .map(GroupPermissionResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(result, None)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}
#[derive(Deserialize, Clone, ToSchema)]
pub struct DeleteGroupPermissionFormData {
    pub group_id: Uuid,
    pub permission_id: Uuid,
}
#[utoipa::path(
    delete,
    path = "/api/group_permissions",
    request_body =DeleteGroupPermissionFormData,
    responses(
        (status = 200, description = "Group permission deleted successfully", body = GroupPermission),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_group_permission_route(
    body: Json<DeleteGroupPermissionFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_group_permission(transaction, body.group_id, body.permission_id).await {
        Ok(result) => Ok(HttpResponse::Ok().json(GroupPermissionResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/api/group_permissions/group/{group_id}",
    params(
        ("group_id" = Uuid, Path, description = "Group ID")
    ),
    responses(
        (status = 200, description = "Group permissions deleted successfully", body = Vec<GroupPermission>),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_group_permissions_by_group_id_route(
    group_id: Path<Uuid>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_group_permissions_by_group_id(transaction, *group_id).await {
        Ok(group_permissions) => {
            let result: Vec<GroupPermissionResponse> = group_permissions
                .into_iter()
                .map(GroupPermissionResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(result, None)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/api/group_permissions/permission/{permission_id}",
    params(
        ("permission_id" = Uuid, Path, description = "Permission ID")
    ),
    responses(
        (status = 200, description = "Group permissions deleted successfully", body = Vec<GroupPermission>),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_group_permissions_by_permission_id_route(
    permission_id: Path<Uuid>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_group_permissions_by_permission_id(transaction, *permission_id).await {
        Ok(group_permissions) => {
            let result: Vec<GroupPermissionResponse> = group_permissions
                .into_iter()
                .map(GroupPermissionResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(result, None)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}
pub fn group_permissions_routes(cfg: &mut web::ServiceConfig) {
    let config = get_configuration().unwrap();
    cfg.service(
        web::scope(format!("/{}", config.routes.group_permissions).as_str())
            .route("", web::post().to(insert_group_permission_route))
            .route("", web::get().to(get_all_group_permissions_route))
            .route(
                "/group/{group_id}",
                web::get().to(get_group_permissions_by_group_id_route),
            )
            .route(
                "/permission/{permission_id}",
                web::get().to(get_group_permissions_by_permission_id_route),
            )
            .route("", web::delete().to(delete_group_permission_route))
            .route(
                "/group/{group_id}",
                web::delete().to(delete_group_permissions_by_group_id_route),
            )
            .route(
                "/permission/{permission_id}",
                web::delete().to(delete_group_permissions_by_permission_id_route),
            ),
    );
}
