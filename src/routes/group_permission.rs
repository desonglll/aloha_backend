use crate::dto::query::DtoQuery;
use crate::error::AlohaError;
use crate::mappers::group_permission::{
    delete_group_permission, delete_group_permissions_by_group_id,
    delete_group_permissions_by_permission_id, get_all_group_permissions,
    get_group_permissions_by_group_id, get_group_permissions_by_permission_id,
    insert_group_permission,
};
use crate::models::group_permission::GroupPermission;
use actix_web::web::{Data, Json, Path, Query};
use actix_web::HttpResponse;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub struct FormData {
    pub group_id: Uuid,
    pub permission_id: Uuid,
}

/// Create a new group permission
///
/// # API Documentation
///
/// ## POST /api/group-permissions
///
/// Creates a new group permission mapping.
///
/// ### Request Body
/// ```json
/// {
///     "group_id": "uuid",
///     "permission_id": "uuid"
/// }
/// ```
///
/// ### Response
/// - 200 OK: Returns the created group permission
/// ```json
/// {
///     "group_id": "uuid",
///     "permission_id": "uuid",
///     "created_at": "datetime"
/// }
/// ```
/// - 400 Bad Request: Database error
#[utoipa::path(
    post,
    path = "/group-permissions",
    request_body = FormData,
    responses(
        (status = 200, description = "Group permission created successfully", body = GroupPermission),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn insert_group_permission_route(
    body: Json<FormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    let group_permission = GroupPermission::new(body.group_id, body.permission_id);
    match insert_group_permission(transaction, &group_permission).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get all group permissions with pagination
///
/// # API Documentation
///
/// ## GET /api/group-permissions
///
/// Retrieves all group permissions with optional pagination.
///
/// ### Query Parameters
/// - page: Page number (optional)
/// - size: Items per page (optional)
///
/// ### Response
/// - 200 OK: Returns list of group permissions
/// ```json
/// {
///     "data": [
///         {
///             "group_id": "uuid",
///             "permission_id": "uuid",
///             "created_at": "datetime"
///         }
///     ],
///     "pagination": {
///         "total": 1,
///         "page": 1,
///         "size": 10,
///         "pages": 1
///     }
/// }
/// ```
/// - 400 Bad Request: Database error
#[utoipa::path(
    get,
    path = "/group-permissions",
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
    query: Query<DtoQuery>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_all_group_permissions(transaction, query.0).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get group permissions by group ID
///
/// # API Documentation
///
/// ## GET /api/group-permissions/group/{group_id}
///
/// Retrieves all permissions for a specific group.
///
/// ### Path Parameters
/// - group_id: UUID of the group
///
/// ### Response
/// - 200 OK: Returns list of group permissions
/// ```json
/// [
///     {
///         "group_id": "uuid",
///         "permission_id": "uuid",
///         "created_at": "datetime"
///     }
/// ]
/// ```
/// - 400 Bad Request: Database error
#[utoipa::path(
    get,
    path = "/group-permissions/group/{group_id}",
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
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get group permissions by permission ID
///
/// # API Documentation
///
/// ## GET /api/group-permissions/permission/{permission_id}
///
/// Retrieves all groups that have a specific permission.
///
/// ### Path Parameters
/// - permission_id: UUID of the permission
///
/// ### Response
/// - 200 OK: Returns list of group permissions
/// ```json
/// [
///     {
///         "group_id": "uuid",
///         "permission_id": "uuid",
///         "created_at": "datetime"
///     }
/// ]
/// ```
/// - 400 Bad Request: Database error
#[utoipa::path(
    get,
    path = "/group-permissions/permission/{permission_id}",
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
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Delete a group permission
///
/// # API Documentation
///
/// ## DELETE /api/group-permissions
///
/// Deletes a specific group permission mapping.
///
/// ### Request Body
/// ```json
/// {
///     "group_id": "uuid",
///     "permission_id": "uuid"
/// }
/// ```
///
/// ### Response
/// - 200 OK: Returns the deleted group permission
/// ```json
/// {
///     "group_id": "uuid",
///     "permission_id": "uuid",
///     "created_at": "datetime"
/// }
/// ```
/// - 400 Bad Request: Database error
#[utoipa::path(
    delete,
    path = "/group-permissions",
    request_body = FormData,
    responses(
        (status = 200, description = "Group permission deleted successfully", body = GroupPermission),
        (status = 400, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_group_permission_route(
    body: Json<FormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_group_permission(transaction, body.group_id, body.permission_id).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Delete all permissions for a group
///
/// # API Documentation
///
/// ## DELETE /api/group-permissions/group/{group_id}
///
/// Deletes all permission mappings for a specific group.
///
/// ### Path Parameters
/// - group_id: UUID of the group
///
/// ### Response
/// - 200 OK: Returns list of deleted group permissions
/// ```json
/// [
///     {
///         "group_id": "uuid",
///         "permission_id": "uuid",
///         "created_at": "datetime"
///     }
/// ]
/// ```
/// - 400 Bad Request: Database error
#[utoipa::path(
    delete,
    path = "/group-permissions/group/{group_id}",
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
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Delete all groups for a permission
///
/// # API Documentation
///
/// ## DELETE /api/group-permissions/permission/{permission_id}
///
/// Deletes all group mappings for a specific permission.
///
/// ### Path Parameters
/// - permission_id: UUID of the permission
///
/// ### Response
/// - 200 OK: Returns list of deleted group permissions
/// ```json
/// [
///     {
///         "group_id": "uuid",
///         "permission_id": "uuid",
///         "created_at": "datetime"
///     }
/// ]
/// ```
/// - 400 Bad Request: Database error
#[utoipa::path(
    delete,
    path = "/group-permissions/permission/{permission_id}",
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
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}
