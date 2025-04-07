use crate::dto::query::DtoQuery;
use crate::error::AlohaError;
use crate::mappers::user_group::{
    delete_user_group_by_id, get_all_groups, get_group_by_id, insert_user_group, update_user_group,
};
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

/// Create a new user group
///
/// # API Documentation
///
/// ## POST /api/user-groups
///
/// Creates a new user group with the specified name.
///
/// ### Request Body
/// ```json
/// {
///     "group_name": "string"
/// }
/// ```
///
/// ### Response
/// - 200 OK: Returns the created user group
/// ```json
/// {
///     "id": "uuid",
///     "group_name": "string",
///     "created_at": "datetime",
///     "updated_at": "datetime"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn insert_user_group_route(
    body: Json<FormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let group_name = body.group_name.clone();
    let transaction = pool.begin().await.unwrap();
    let user_group = UserGroup::new(group_name.clone());
    match insert_user_group(transaction, &user_group).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get all user groups with pagination
///
/// # API Documentation
///
/// ## GET /api/user-groups
///
/// Retrieves all user groups with optional pagination and filtering.
///
/// ### Query Parameters
/// - page: Page number (optional)
/// - size: Items per page (optional)
/// - sort: Sort field (optional)
/// - order: Sort order (asc/desc) (optional)
///
/// ### Response
/// - 200 OK: Returns list of user groups
/// ```json
/// [
///     {
///         "id": "uuid",
///         "group_name": "string",
///         "created_at": "datetime",
///         "updated_at": "datetime"
///     }
/// ]
/// ```
/// - 500 Internal Server Error: Database error
pub async fn get_all_user_groups_route(
    query: web::Query<DtoQuery>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_all_groups(transaction, query.0).await {
        Ok(user_groups) => Ok(HttpResponse::Ok().json(user_groups)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get a specific user group by ID
///
/// # API Documentation
///
/// ## GET /api/user-groups/{id}
///
/// Retrieves a specific user group by its ID.
///
/// ### Path Parameters
/// - id: UUID of the user group
///
/// ### Response
/// - 200 OK: Returns the user group
/// ```json
/// {
///     "id": "uuid",
///     "group_name": "string",
///     "created_at": "datetime",
///     "updated_at": "datetime"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn get_user_group_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let user_id = id.0;
    let transaction = pool.begin().await.unwrap();
    match get_group_by_id(transaction, user_id).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Update an existing user group
///
/// # API Documentation
///
/// ## PUT /api/user-groups
///
/// Updates an existing user group.
///
/// ### Request Body
/// ```json
/// {
///     "id": "uuid",
///     "group_name": "string",
///     "created_at": "datetime",
///     "updated_at": "datetime"
/// }
/// ```
///
/// ### Response
/// - 200 OK: Returns the updated user group
/// ```json
/// {
///     "id": "uuid",
///     "group_name": "string",
///     "created_at": "datetime",
///     "updated_at": "datetime"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn update_user_group_route(
    body: Json<UserGroup>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match update_user_group(transaction, &body).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Delete a user group by ID
///
/// # API Documentation
///
/// ## DELETE /api/user-groups/{id}
///
/// Deletes a specific user group by its ID.
///
/// ### Path Parameters
/// - id: UUID of the user group to delete
///
/// ### Response
/// - 200 OK: Returns the deleted user group
/// ```json
/// {
///     "id": "uuid",
///     "group_name": "string",
///     "created_at": "datetime",
///     "updated_at": "datetime"
/// }
/// ```
/// - 500 Internal Server Error: Database error
pub async fn delete_user_group_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_user_group_by_id(transaction, id.0).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}
