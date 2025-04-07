use crate::helpers::spawn_app;
use aloha_backend::mappers::tweet::insert_tweet;
use aloha_backend::mappers::user::insert_user;
use aloha_backend::mappers::user_group::insert_user_group;
use aloha_backend::models::tweet::Tweet;
use aloha_backend::models::user::User;
use aloha_backend::models::user_group::UserGroup;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn insert_tweet_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    // First create a user group
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    // Then create a user
    transaction = app.db_pool.begin().await.unwrap();
    let mut user = User::default_test();
    user.user_group_id = Some(user_group_result.id);
    let user_result = insert_user(transaction, &user).await.unwrap();

    let body = serde_json::json!({
        "content": "This is a test tweet",
        "user_id": user_result.id
    });
    let mock_server = MockServer::start().await;
    Mock::given(path("/tweet"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.post_tweet(&body).await.unwrap();

    assert_eq!(response.content, "This is a test tweet");
    assert_eq!(response.user_id, user_result.id);
}

#[tokio::test]
async fn get_all_tweets_returns_a_200() {
    let app = spawn_app().await;

    // First create a user group
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    // Then create a user
    transaction = app.db_pool.begin().await.unwrap();
    let mut user = User::default_test();
    user.user_group_id = Some(user_group_result.id);
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Create some tweets
    transaction = app.db_pool.begin().await.unwrap();
    let tweets = Tweet::default_vec_test(user_result.id, Some(3));

    for tweet in &tweets {
        insert_tweet(transaction, tweet).await.unwrap();
        transaction = app.db_pool.begin().await.unwrap();
    }

    let mock_server = MockServer::start().await;
    Mock::given(path("/tweets"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.get_all_tweets().await.unwrap();
    assert!(!response.data.is_empty());
}

#[tokio::test]
async fn get_tweet_returns_a_200_for_valid_id() {
    let app = spawn_app().await;

    // First create a user group
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    // Then create a user
    transaction = app.db_pool.begin().await.unwrap();
    let mut user = User::default_test();
    user.user_group_id = Some(user_group_result.id);
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Create a tweet
    transaction = app.db_pool.begin().await.unwrap();
    let default_tweet = Tweet::default_test(user_result.id);
    let insert_result = insert_tweet(transaction, &default_tweet).await.unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/tweet/{id}"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.get_tweet_by_id(insert_result.id).await.unwrap();
    assert_eq!(response.id, insert_result.id);
    assert_eq!(response.content, insert_result.content);
}

#[tokio::test]
async fn get_tweets_by_user_returns_a_200_for_valid_user_id() {
    let app = spawn_app().await;

    // First create a user group
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    // Then create a user
    transaction = app.db_pool.begin().await.unwrap();
    let mut user = User::default_test();
    user.user_group_id = Some(user_group_result.id);
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Create some tweets
    transaction = app.db_pool.begin().await.unwrap();
    let tweets = Tweet::default_vec_test(user_result.id, Some(3));

    for tweet in &tweets {
        insert_tweet(transaction, tweet).await.unwrap();
        transaction = app.db_pool.begin().await.unwrap();
    }

    let mock_server = MockServer::start().await;
    Mock::given(path("/user/{user_id}/tweets"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.get_tweets_by_user_id(user_result.id).await.unwrap();
    assert!(!response.data.is_empty());
}

#[tokio::test]
async fn update_tweet_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    // First create a user group
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    // Then create a user
    transaction = app.db_pool.begin().await.unwrap();
    let mut user = User::default_test();
    user.user_group_id = Some(user_group_result.id);
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Create a tweet
    transaction = app.db_pool.begin().await.unwrap();
    let default_tweet = Tweet::default_test(user_result.id);
    let insert_result = insert_tweet(transaction, &default_tweet).await.unwrap();

    let body = serde_json::json!({
        "content": "Updated tweet content"
    });

    let mock_server = MockServer::start().await;
    Mock::given(path("/tweet/{id}"))
        .and(method("PUT"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.put_tweet(insert_result.id, &body).await.unwrap();
    assert_eq!(response.content, "Updated tweet content");
}

#[tokio::test]
async fn delete_tweet_returns_a_200_for_valid_id() {
    let app = spawn_app().await;

    // First create a user group
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    // Then create a user
    transaction = app.db_pool.begin().await.unwrap();
    let mut user = User::default_test();
    user.user_group_id = Some(user_group_result.id);
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Create a tweet
    transaction = app.db_pool.begin().await.unwrap();
    let default_tweet = Tweet::default_test(user_result.id);
    let insert_result = insert_tweet(transaction, &default_tweet).await.unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/tweet/{id}"))
        .and(method("DELETE"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.delete_tweet(insert_result.id).await.unwrap();
    assert_eq!(response.id, insert_result.id);
}
