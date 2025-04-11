use crate::configuration::get_configuration;
use crate::dto::query::{DtoQuery, UserFilterQuery};
use crate::dto::response::DtoResponse;
use crate::error::AlohaError;
use crate::mappers::user::{
    delete_user_by_id, delete_users_by_ids, get_all_users, get_user_by_id, insert_user, update_user,
};
use crate::models::user::{User, UserResponse};
use actix_web::web::{Data, Json};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_qs::actix::QsQuery;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Clone, ToSchema)]
pub struct CreateUserFormData {
    username: String,
    password: String,
    user_group_id: Option<Uuid>,
}

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
    query: QsQuery<DtoQuery<UserFilterQuery>>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    match get_all_users(transaction, query.into_inner()).await {
        Ok(users) => {
            let user_responses: Vec<UserResponse> =
                users.data.into_iter().map(UserResponse::from).collect();
            Ok(HttpResponse::Ok().json(DtoResponse::new(user_responses, users.pagination)))
        }
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

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
        Ok(Some(result)) => Ok(HttpResponse::Ok().json(UserResponse::from(result))),
        Ok(None) => Err(AlohaError::DatabaseError(
            "User Group not found".to_string(),
        )),
        Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
    }
}

#[derive(Deserialize, Debug, Clone, ToSchema)]
pub struct PutUserFormData {
    pub id: Uuid,
    pub username: String,
    pub password: Option<String>,
    pub user_group_id: Option<Uuid>,
}

#[utoipa::path(
    put,
    path = "/api/users",
    request_body = PutUserFormData,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 500, description = "Database error", body = AlohaError)
    )
)]
pub async fn update_user_route(
    body: Json<PutUserFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, AlohaError> {
    let transaction = pool.begin().await.unwrap();
    dbg!(&body);

    let find_user = match get_user_by_id(transaction, body.0.id).await {
        Ok(user) => user,
        Err(e) => return Err(AlohaError::DatabaseError(e.to_string())),
    };
    match find_user {
        Some(mut u) => {
            let transaction = pool.begin().await.unwrap();
            u.username = body.username.clone();
            u.user_group_id = body.user_group_id;
            if let Some(password) = body.password.clone() {
                u.password_hash = password;
            }

            match update_user(transaction, &u).await {
                Ok(result) => Ok(HttpResponse::Ok().json(UserResponse::from(result))),
                Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
            }
        }
        None => {
            let transaction = pool.begin().await.unwrap();
            let password_hash = body.password.clone().unwrap();
            let user = User::new(body.username.clone(), password_hash, body.user_group_id);
            match insert_user(transaction, &user).await {
                Ok(result) => Ok(HttpResponse::Ok().json(UserResponse::from(result))),
                Err(e) => Err(AlohaError::DatabaseError(e.to_string())),
            }
        }
    }
}

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
            .route("", web::put().to(update_user_route))
            .route("", web::get().to(get_all_users_route))
            .route("/{id}", web::delete().to(delete_user_route))
            .route("", web::delete().to(delete_users_route)),
    );
}
