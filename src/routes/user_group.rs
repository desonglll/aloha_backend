use crate::configuration::get_configuration;
use crate::dto::query::DtoQuery;
use crate::dto::response::DtoResponse;
use crate::error::AlohaError;
use crate::mappers::user_group::{
    delete_user_group_by_id, get_all_groups, get_group_by_id, insert_user_group, update_user_group,
};
use crate::models::user_group::UserGroup;
use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Clone, ToSchema)]
pub(crate) struct FormData {
    group_name: String,
}

/// Create a new user group
///
/// # API Documentation
///
/// ## POST /api/user_groups
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
#[utoipa::path(
    post,
    path = "/api/user_groups",
    request_body = FormData,
    responses(
        (status = 200, description = "User group created successfully", body = UserGroup),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
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
/// ## GET /api/user_groups
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
/// ## GET /api/user_groups/{id}
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
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Update an existing user group
///
/// # API Documentation
///
/// ## PUT /api/user_groups
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
#[utoipa::path(
    put,
    path = "/api/user_groups",
    request_body = UserGroup,
    responses(
        (status = 200, description = "User group updated successfully", body = UserGroup),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
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
/// ## DELETE /api/user_groups/{id}
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
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
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
