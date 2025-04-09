use crate::configuration::get_configuration;
use crate::dto::query::DtoQuery;
use crate::dto::response::DtoResponse;
use crate::error::AlohaError;
use crate::mappers::permission::{
    delete_permission_by_id, get_all_permissions, get_permission_by_id, insert_permission,
    update_permission,
};
use crate::models::permission::{Permission, PermissionResponse};
use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Clone, ToSchema)]
pub struct CreatePermissionFormData {
    pub name: String,
    pub description: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/permissions",
    request_body = CreatePermissionFormData,
    responses(
        (status = 200, description = "Permission created successfully", body = Permission),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn insert_permission_route(
    body: Json<CreatePermissionFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    let permission = Permission::from(body.0);
    match insert_permission(transaction, &permission).await {
        Ok(result) => Ok(HttpResponse::Ok().json(PermissionResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/permissions",
    params(
        ("page" = Option<i32>, Query, description = "Page number"),
        ("size" = Option<i32>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Permissions retrieved successfully", body = DtoResponse<Vec<Permission>>),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_all_permissions_route(
    query: web::Query<DtoQuery>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_all_permissions(transaction, query.0).await {
        Ok(permissions) => {
            let result: Vec<PermissionResponse> = permissions
                .data
                .into_iter()
                .map(PermissionResponse::from)
                .collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(result, permissions.pagination)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/permissions/{id}",
    params(
        ("id" = Uuid, Path, description = "Permission ID")
    ),
    responses(
        (status = 200, description = "Permission retrieved successfully", body = Permission),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_permission_by_id_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let permission_id = id.0;
    let transaction = pool.begin().await.unwrap();
    match get_permission_by_id(transaction, permission_id).await {
        Ok(Some(result)) => Ok(HttpResponse::Ok().json(PermissionResponse::from(result))),
        Ok(None) => Err(AlohaError::DatabaseError(
            "Permission not found".to_string(),
        )),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct PutPermissionFormData {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
#[utoipa::path(
    put,
    path = "/api/permissions",
    request_body = PutPermissionFormData,
    responses(
        (status = 200, description = "Permission updated successfully", body = Permission),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn update_permission_by_id_route(
    body: Json<PutPermissionFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();

    let find_permission = get_permission_by_id(transaction, body.0.id).await.unwrap();
    match find_permission {
        Some(mut permission) => {
            permission.name = body.0.name.clone();
            permission.description = body.0.description.clone();

            let transaction = pool.begin().await.unwrap();

            match update_permission(transaction, &permission).await {
                Ok(result) => Ok(HttpResponse::Ok().json(PermissionResponse::from(result))),
                Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
            }
        }
        None => {
            let permission = Permission::new(body.0.name, body.0.description);
            let transaction = pool.begin().await.unwrap();
            match insert_permission(transaction, &permission).await {
                Ok(result) => Ok(HttpResponse::Ok().json(PermissionResponse::from(result))),
                Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
            }
        }
    }
}
#[utoipa::path(
    delete,
    path = "/api/permissions/{id}",
    params(
        ("id" = Uuid, Path, description = "Permission ID")
    ),
    responses(
        (status = 200, description = "Permission deleted successfully", body = Permission),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_permission_by_id_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_permission_by_id(transaction, id.0).await {
        Ok(result) => Ok(HttpResponse::Ok().json(PermissionResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}
pub fn permission_routes(cfg: &mut web::ServiceConfig) {
    let config = get_configuration().unwrap();
    cfg.service(
        web::scope(format!("/{}", config.routes.permissions).as_str())
            .route("", web::post().to(insert_permission_route))
            .route("/{id}", web::get().to(get_permission_by_id_route))
            .route("/{id}", web::put().to(update_permission_by_id_route))
            .route("", web::get().to(get_all_permissions_route))
            .route("/{id}", web::delete().to(delete_permission_by_id_route)),
    );
}
