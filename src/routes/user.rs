use crate::configuration::get_configuration;
use crate::dto::query::DtoQuery;
use crate::dto::response::DtoResponse;
use crate::error::AlohaError;
use crate::mappers::user::{
    delete_user_by_id, delete_users_by_ids, get_all_users, get_user_by_id, insert_user, update_user,
};
use crate::models::user::{User, UserResponse};
use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Clone, ToSchema)]
pub(crate) struct CreateUserFormData {
    username: String,
    password: String,
    user_group_id: Option<Uuid>,
}

/// Create a new user
///
/// # API Documentation
///
/// ## POST /api/users
///
/// Creates a new user with the provided information.
///
/// ### Request Body
/// ```json
/// {
///     "username": "string",
///     "password": "string",
///     "user_group_id": "uuid" (optional)
/// }
/// ```
///
/// ### Response
/// - 200 OK: Returns the created user
/// ```json
/// {
///     "id": "uuid",
///     "username": "string",
///     "created_at": "datetime",
///     "user_group_id": "uuid" (optional)
/// }
/// ```
/// - 500 Internal Server Error: Database error
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserFormData,
    responses(
        (status = 200, description = "User created successfully", body = UserResponse),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn insert_user_route(
    body: Json<CreateUserFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    // In a real application, you would hash the password here
    let password_hash = body.password.clone(); // This should be properly hashed in production
    let transaction = pool.begin().await.unwrap();
    let user = User::new(body.username.clone(), password_hash, body.user_group_id);
    match insert_user(transaction, &user).await {
        Ok(result) => Ok(HttpResponse::Ok().json(UserResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get all users with pagination
///
/// # API Documentation
///
/// ## GET /api/users
///
/// Retrieves all users with optional pagination and filtering.
///
/// ### Query Parameters
/// - page: Page number (optional)
/// - size: Items per page (optional)
/// - sort: Sort field (optional)
/// - order: Sort order (asc/desc) (optional)
///
/// ### Response
/// - 200 OK: Returns list of users
/// ```json
/// [
///     {
///         "id": "uuid",
///         "username": "string",
///         "created_at": "datetime",
///         "user_group_id": "uuid" (optional)
///     }
/// ]
/// ```
/// - 500 Internal Server Error: Database error
#[utoipa::path(
    get,
    path = "/api/users",
    params(
        ("page" = Option<i32>, Query, description = "Page number"),
        ("size" = Option<i32>, Query, description = "Page size"),
        ("sort" = Option<String>, Query, description = "Sort field"),
        ("order" = Option<String>, Query, description = "Sort order (asc/desc)")
    ),
    responses(
        (status = 200, description = "Users retrieved successfully", body = DtoResponse<Vec<UserResponse>>),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_all_users_route(
    query: web::Query<DtoQuery>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_all_users(transaction, query.0).await {
        Ok(users) => {
            let user_responses: Vec<UserResponse> =
                users.data.into_iter().map(UserResponse::from).collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(user_responses, users.pagination)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Get a specific user by ID
///
/// # API Documentation
///
/// ## GET /api/users/{id}
///
/// Retrieves a specific user by their ID.
///
/// ### Path Parameters
/// - id: UUID of the user
///
/// ### Response
/// - 200 OK: Returns the user
/// ```json
/// {
///     "id": "uuid",
///     "username": "string",
///     "created_at": "datetime",
///     "user_group_id": "uuid" (optional)
/// }
/// ```
/// - 500 Internal Server Error: Database error
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = UserResponse),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn get_user_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let user_id = id.0;
    let transaction = pool.begin().await.unwrap();
    match get_user_by_id(transaction, user_id).await {
        Ok(result) => Ok(HttpResponse::Ok().json(UserResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[derive(Deserialize, Clone, ToSchema)]
pub(crate) struct UpdateUserFormData {
    username: String,
    password: Option<String>,
    user_group_id: Option<Uuid>,
}

/// Update an existing user
///
/// # API Documentation
///
/// ## PUT /api/users/{id}
///
/// Updates an existing user.
///
/// ### Path Parameters
/// - id: UUID of the user to update
///
/// ### Request Body
/// ```json
/// {
///     "username": "string",
///     "password": "string" (optional),
///     "user_group_id": "uuid" (optional)
/// }
/// ```
///
/// ### Response
/// - 200 OK: Returns the updated user
/// ```json
/// {
///     "id": "uuid",
///     "username": "string",
///     "created_at": "datetime",
///     "user_group_id": "uuid" (optional)
/// }
/// ```
/// - 500 Internal Server Error: Database error
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserFormData,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn update_user_route(
    id: web::Path<(Uuid,)>,
    body: Json<UpdateUserFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let user_id = id.0;
    let transaction = pool.begin().await.unwrap();

    // First get the existing user to preserve password if not provided
    let existing_user = match get_user_by_id(transaction, user_id).await {
        Ok(user) => user,
        Err(e) => return Err(AlohaError::DatabaseError(e.to_string())),
    };

    // Start a new transaction
    let transaction = pool.begin().await.unwrap();

    // Update the user with new values
    let password_hash = match &body.password {
        Some(password) => password.clone(), // Again, this should be hashed in production
        None => existing_user.password_hash,
    };

    let updated_user = User {
        id: user_id,
        username: body.username.clone(),
        password_hash,
        created_at: existing_user.created_at,
        user_group_id: body.user_group_id,
    };

    match update_user(transaction, &updated_user).await {
        Ok(result) => Ok(HttpResponse::Ok().json(UserResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}
/// Delete multiple users by their IDs
///
/// # API Documentation
///
/// ## DELETE /api/users
///
/// Deletes multiple users by their IDs.
///
/// ### Request Body
/// ```json
/// [
///     "uuid",
///     "uuid"
/// ]
/// ```
///
/// ### Response
/// - 200 OK: Returns the number of deleted users
/// ```json
/// {
///     "count": 2
/// }
/// ```
/// - 500 Internal Server Error: Database error
#[utoipa::path(
    delete,
    path = "/api/users",
    request_body = Vec<Uuid>,
    responses(
        (status = 200, description = "Users deleted successfully", body = i64),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_users_route(
    body: Json<Vec<Uuid>>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match delete_users_by_ids(transaction, body.into_inner()).await {
        Ok(result) => Ok(HttpResponse::Ok().json(
            result
                .into_iter()
                .map(UserResponse::from)
                .collect::<Vec<_>>(),
        )),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

/// Delete a specific user by ID
///
/// # API Documentation
///
/// ## DELETE /api/users/{id}
///
/// Deletes a specific user by their ID.
///
/// ### Path Parameters
/// - id: UUID of the user
///
/// ### Response
/// - 200 OK: Returns the deleted user
/// ```json
/// {
///     "id": "uuid",
///     "username": "string",
///     "created_at": "datetime",
///     "user_group_id": "uuid" (optional)
/// }
/// ```
/// - 500 Internal Server Error: Database error
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = UserResponse),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn delete_user_route(
    id: web::Path<(Uuid,)>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let user_id = id.0;
    let transaction = pool.begin().await.unwrap();
    match delete_user_by_id(transaction, user_id).await {
        Ok(result) => Ok(HttpResponse::Ok().json(UserResponse::from(result))),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}
pub fn user_routes(cfg: &mut web::ServiceConfig) {
    let config = get_configuration().unwrap();
    cfg.service(
        web::scope(format!("/{}", config.routes.users).as_str())
            .route("", web::post().to(insert_user_route))
            .route("/{id}", web::get().to(get_user_route))
            .route("/{id}", web::put().to(update_user_route))
            .route("", web::get().to(get_all_users_route))
            .route("/{id}", web::delete().to(delete_user_route))
            .route("", web::delete().to(delete_users_route)),
    );
}
