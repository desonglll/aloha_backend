use crate::helpers::spawn_app;
use aloha_backend::dto::query::{DtoQuery, TweetFilterQuery};
use aloha_backend::mappers::tweet::{get_all_tweets, insert_tweet};
use aloha_backend::mappers::user::insert_user;
use aloha_backend::models::tweet::Tweet;
use aloha_backend::models::user::User;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn insert_tweet_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    // First create a user
    let transaction = app.db_pool.begin().await.unwrap();
    let user = User::default_test();
    let user_result = insert_user(transaction, &user).await.unwrap();

    let body = serde_json::json!({
        "content": "Test tweet content",
        "user_id": user_result.id
    });

    let mock_server = MockServer::start().await;
    Mock::given(path("/api/tweets"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.post_tweet(&body).await.unwrap();

    assert_eq!(response.content, "Test tweet content");
    assert_eq!(response.user_id, user_result.id);
}

#[tokio::test]
async fn get_all_tweets_returns_a_200() {
    let app = spawn_app().await;

    // First create a user
    let transaction = app.db_pool.begin().await.unwrap();
    let user = User::default_test();
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Create tweets one at a time to ensure they are properly committed
    for i in 1..=3 {
        let transaction = app.db_pool.begin().await.unwrap();
        let mut tweet = Tweet::default_test(user_result.id);
        tweet.content = format!("Test tweet content {}", i);
        let _ = insert_tweet(transaction, &tweet).await.unwrap();
    }

    // Sleep briefly to ensure all transactions are committed
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Verify the tweets exist in the database before testing the route
    let transaction = app.db_pool.begin().await.unwrap();
    let query = DtoQuery::<TweetFilterQuery>::default_query();
    let db_result = get_all_tweets(transaction, query).await.unwrap();
    assert!(
        db_result.data.len() >= 3,
        "Failed to create test tweets in the database"
    );

    let mock_server = MockServer::start().await;
    Mock::given(path("/api/tweets"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.get_all_tweets().await.unwrap();
    assert!(
        !response.data.is_empty(),
        "API response should contain tweets"
    );

    // Check if we can find at least one tweet with the expected pattern
    let found_test_tweet = response
        .data
        .iter()
        .any(|t| t.content.starts_with("Test tweet content") && t.user_id == user_result.id);
    assert!(found_test_tweet, "Expected to find at least one test tweet");
}

#[tokio::test]
async fn get_tweet_returns_a_200_for_valid_id() {
    let app = spawn_app().await;

    // First create a user
    let transaction = app.db_pool.begin().await.unwrap();
    let user = User::default_test();
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Now create a tweet with the user ID
    let transaction = app.db_pool.begin().await.unwrap();
    let tweet = Tweet::default_test(user_result.id);
    let insert_result = insert_tweet(transaction, &tweet).await.unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/api/tweets/{id}"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.get_tweet_by_id(insert_result.id).await.unwrap();
    assert_eq!(response.id, insert_result.id);
    assert_eq!(response.content, insert_result.content);
    assert_eq!(response.user_id, user_result.id);
}

#[tokio::test]
async fn update_tweet_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    // First create a user
    let transaction = app.db_pool.begin().await.unwrap();
    let user = User::default_test();
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Now create a tweet with the user ID
    let transaction = app.db_pool.begin().await.unwrap();
    let tweet = Tweet::default_test(user_result.id);
    let insert_result = insert_tweet(transaction, &tweet).await.unwrap();

    let body = serde_json::json!({
        "id": insert_result.id,
        "content": "Updated tweet content"
    });

    let mock_server = MockServer::start().await;
    Mock::given(path("/api/tweets/{id}"))
        .and(method("PUT"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.put_tweet(&body).await.unwrap();
    assert_eq!(response.id, insert_result.id);
    assert_eq!(response.content, "Updated tweet content");
    assert_eq!(response.user_id, user_result.id);
}

#[tokio::test]
async fn delete_tweets_returns_a_200_for_valid_ids() {
    let app = spawn_app().await;

    // First create a user
    let transaction = app.db_pool.begin().await.unwrap();
    let user = User::default_test();
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Create first tweet
    let transaction = app.db_pool.begin().await.unwrap();
    let mut tweet1 = Tweet::default_test(user_result.id);
    tweet1.content = "First test tweet".to_string();
    let tweet1_result = insert_tweet(transaction, &tweet1).await.unwrap();

    // Create second tweet
    let transaction = app.db_pool.begin().await.unwrap();
    let mut tweet2 = Tweet::default_test(user_result.id);
    tweet2.content = "Second test tweet".to_string();
    let tweet2_result = insert_tweet(transaction, &tweet2).await.unwrap();

    let tweet_ids = vec![tweet1_result.id, tweet2_result.id];

    let mock_server = MockServer::start().await;
    Mock::given(path("/api/tweets"))
        .and(method("DELETE"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.delete_tweets(&tweet_ids).await.unwrap();
    assert_eq!(response.len(), 2);
    assert!(response.iter().any(|t| t.id == tweet1_result.id));
    assert!(response.iter().any(|t| t.id == tweet2_result.id));
}

#[tokio::test]
async fn delete_tweet_returns_a_200_for_valid_id() {
    let app = spawn_app().await;

    // First create a user
    let transaction = app.db_pool.begin().await.unwrap();
    let user = User::default_test();
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Now create a tweet with the user ID
    let transaction = app.db_pool.begin().await.unwrap();
    let tweet = Tweet::default_test(user_result.id);
    let insert_result = insert_tweet(transaction, &tweet).await.unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/api/tweets/{id}"))
        .and(method("DELETE"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.delete_tweet(insert_result.id).await.unwrap();
    assert_eq!(response.id, insert_result.id);
    assert_eq!(response.content, insert_result.content);
    assert_eq!(response.user_id, user_result.id);
}
