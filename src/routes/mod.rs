use actix_web::web;
use auth::auth_routes;
use group_permission::group_permissions_routes;
use health_check::health_check;
use permission::permission_routes;
use serde::Deserialize;
use tweet::tweet_routes;
use user::user_routes;
use user_group::user_group_routes;
use user_permission::user_permissions_routes;

pub mod auth;
pub mod group_permission;
pub mod health_check;
pub mod permission;
pub mod tweet;
pub mod user;
pub mod user_group;
pub mod user_permission;

#[derive(Clone, Debug, Deserialize)]
pub struct Routes {
    pub user_groups: String,
    pub users: String,
    pub permissions: String,
    pub tweets: String,
    pub group_permissions: String,
    pub user_permissions: String,
    pub auth: String,
}
pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(permission_routes)
            .configure(group_permissions_routes)
            .configure(user_group_routes)
            .configure(user_routes)
            .configure(user_permissions_routes)
            .configure(tweet_routes)
            .configure(auth_routes)
            .route("/health", web::get().to(health_check)),
    );
}
