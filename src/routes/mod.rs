use crate::routes::health_check::health_check;
use crate::routes::tweet::{
    delete_tweet_route, get_all_tweets_route, get_tweet_route, get_tweets_by_user_route,
    insert_tweet_route, update_tweet_route,
};
use crate::routes::user::{
    delete_user_route, get_all_users_route, get_user_route, insert_user_route, update_user_route,
};
use crate::routes::user_group::{
    delete_user_group_route, get_all_user_groups_route, get_user_group_route,
    insert_user_group_route, update_user_group_route,
};
use actix_web::web::{delete, get, post, put, ServiceConfig};
use serde::{Deserialize, Serialize};

pub mod health_check;
pub mod tweet;
pub mod user;
pub mod user_group;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Routes {
    health_check: String,
    user_groups: String,
    user_group: String,
    users: String,
    user: String,
    tweets: String,
    tweet: String,
    user_tweets: String,
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    // Health check
    cfg.service(get().to(health_check).route("/health_check"));

    // User group routes
    cfg.service(post().to(insert_user_group_route).route("/user_group"));
    cfg.service(get().to(get_all_user_groups_route).route("/user_groups"));
    cfg.service(get().to(get_user_group_route).route("/user_group/{id}"));
    cfg.service(put().to(update_user_group_route).route("/user_group"));
    cfg.service(
        delete()
            .to(delete_user_group_route)
            .route("/user_group/{id}"),
    );

    // User routes
    cfg.service(post().to(insert_user_route).route("/user"));
    cfg.service(get().to(get_all_users_route).route("/users"));
    cfg.service(get().to(get_user_route).route("/user/{id}"));
    cfg.service(put().to(update_user_route).route("/user/{id}"));
    cfg.service(delete().to(delete_user_route).route("/user/{id}"));

    // Tweet routes
    cfg.service(post().to(insert_tweet_route).route("/tweet"));
    cfg.service(get().to(get_all_tweets_route).route("/tweets"));
    cfg.service(get().to(get_tweet_route).route("/tweet/{id}"));
    cfg.service(
        get()
            .to(get_tweets_by_user_route)
            .route("/user/{user_id}/tweets"),
    );
    cfg.service(put().to(update_tweet_route).route("/tweet/{id}"));
    cfg.service(delete().to(delete_tweet_route).route("/tweet/{id}"));
}
