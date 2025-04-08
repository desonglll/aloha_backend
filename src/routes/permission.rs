use crate::dto::query::DtoQuery;
use crate::error::AlohaError;
use crate::mappers::permission::{
    delete_permission_by_id, get_all_permissions, get_permission_by_id, insert_permission,
    update_permission,
};
use crate::models::permission::Permission;
use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub(crate) struct FormData {
    name: String,
    description: Option<String>,
}

/// Create a new permission
///
/// # API Documentation
///
/// ## POST /api/permissions
///
/// Creates a new permission with the specified name and optional description.
///
/// ### Request Body
/// ```json
/// {
///     "name": "string",
///     "description": "string" (optional)
/// }
/// ```
///
/// ### Response
/// - 200 OK: Returns the created permission
/// ```json
/// {
///     "id": "uuid",
///     "name": "string",
///     "description": "string" (optional),
///     "created_at": "datetime"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn insert_permission_route(
    body: Json<FormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let name = body.name.clone();
    let description = body.description.clone();
    let transaction = pool.begin().await.unwrap();
    let permission = Permission::new(name, description);
    match insert_permission(transaction, &permission).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get all permissions with pagination
///
/// # API Documentation
///
/// ## GET /api/permissions
///
/// Retrieves all permissions with optional pagination and filtering.
///
/// ### Query Parameters
/// - page: Page number (optional)
/// - size: Items per page (optional)
/// - sort: Sort field (optional)
/// - order: Sort order (asc/desc) (optional)
///
/// ### Response
/// - 200 OK: Returns list of permissions
/// ```json
/// [
///     {
///         "id": "uuid",
///         "name": "string",
///         "description": "string" (optional),
///         "created_at": "datetime"
///     }
/// ]
/// ```
/// - 500 Internal Server Error: Database error
pub async fn get_all_permissions_route(
    query: web::Query<DtoQuery>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_all_permissions(transaction, query.0).await {
        Ok(permissions) => Ok(HttpResponse::Ok().json(permissions)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get a specific permission by ID
///
/// # API Documentation
///
/// ## GET /api/permissions/{id}
///
/// Retrieves a specific permission by its ID.
///
/// ### Path Parameters
/// - id: UUID of the permission
///
/// ### Response
/// - 200 OK: Returns the permission
/// ```json
/// {
///     "id": "uuid",
///     "name": "string",
///     "description": "string" (optional),
///     "created_at": "datetime"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn get_permission_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let permission_id = id.0;
    let transaction = pool.begin().await.unwrap();
    match get_permission_by_id(transaction, permission_id).await {
        Ok(Some(result)) => Ok(HttpResponse::Ok().json(result)),
        Ok(None) => Err(AlohaError::DatabaseError(
            "Permission not found".to_string(),
        )),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Update an existing permission
///
/// # API Documentation
///
/// ## PUT /api/permissions
///
/// Updates an existing permission.
///
/// ### Request Body
/// ```json
/// {
///     "id": "uuid",
///     "name": "string",
///     "description": "string" (optional),
///     "created_at": "datetime"
/// }
/// ```
///
/// ### Response
/// - 200 OK: Returns the updated permission
/// ```json
/// {
///     "id": "uuid",
///     "name": "string",
///     "description": "string" (optional),
///     "created_at": "datetime"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn update_permission_route(
    body: Json<Permission>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match update_permission(transaction, &body).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Delete a permission by ID
///
/// # API Documentation
///
/// ## DELETE /api/permissions/{id}
///
/// Deletes a specific permission by its ID.
///
/// ### Path Parameters
/// - id: UUID of the permission to delete
///
/// ### Response
/// - 200 OK: Returns the deleted permission
/// ```json
/// {
///     "id": "uuid",
///     "name": "string",
///     "description": "string" (optional),
///     "created_at": "datetime"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn delete_permission_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_permission_by_id(transaction, id.0).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}
