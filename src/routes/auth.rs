use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use tracing::info;

use crate::{
    configuration::get_configuration,
    error::AlohaError,
    mappers::user::{check_user_password_correct, get_user_by_username},
};

/// 用户登录请求
///
/// 该结构体表示用户登录请求中的数据，包括用户名和密码。
///
/// - `user_name`：用户的用户名，用于身份验证。
/// - `password`：用户的密码，用于身份验证。
#[derive(Serialize, Deserialize, Default)]
// #[serde(rename_all = "camelCase")]
pub struct LoginFormData {
    pub username: String,
    pub password: String,
}

/// # 请求示例数据
/*
```json
curl -X POST localhost:8000/api/login \
   -H "Content-Type: application/json" \
   -d '{
        "userName": "root",
        "password": "070011"
    }'
```
*/
pub async fn login(
    session: Session,
    pool: web::Data<Pool<sqlx::Postgres>>,
    body: web::Json<LoginFormData>,
) -> Result<HttpResponse, AlohaError> {
    let mut transaction = pool.begin().await.unwrap();
    // Extract user credentials from the request
    tracing::log::debug!("Request login");
    let username = body.username.clone();
    let password_hash = body.password.clone();

    let user = get_user_by_username(&mut transaction, &username)
        .await
        .unwrap();

    match check_user_password_correct(&mut transaction, user.id, password_hash).await {
        Ok(true) => {
            tracing::log::debug!("Insert session data");
            // Store the user ID in the session
            session.insert("username", user.username.clone()).unwrap();
            session.insert("user_id", user.id).unwrap();

            let result = session.entries().to_owned();

            Ok(HttpResponse::Ok().json(result))
        }
        Ok(false) => {
            // Password is incorrect
            Ok(HttpResponse::Unauthorized().body(AlohaError::UserPasswordInvalid.to_string()))
        }
        Err(e) => {
            // Handle any errors that occurred during password check
            Err(AlohaError::RequestParameterInvalid(e.to_string()))
        }
    }
}

pub async fn logout(session: Session) -> Result<HttpResponse, AlohaError> {
    // Attempt to retrieve the `user_name` from the session
    if let Some(_user_name) = session.get::<String>("user_name").unwrap() {
        session.purge();
        let result = session.entries().to_owned();
        tracing::log::debug!("Logout successful: {:?}", result);
        Ok(HttpResponse::Ok().json(result))
    } else {
        Ok(HttpResponse::Ok()
            .json("Attempt to log out failed: no user found in session.".to_string()))
    }
}

pub async fn check_login(session: &Session) -> Result<bool, AlohaError> {
    match session.get::<String>("username") {
        Ok(_user_name) => Ok(true),
        _ => Ok(false),
    }
}
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    let config = get_configuration().unwrap();
    cfg.service(
        web::scope(format!("/{}", config.routes.auth).as_str())
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout)),
    );
}
