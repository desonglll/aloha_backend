use crate::helpers::spawn_app;
use aloha_backend::mappers::user::insert_user;
use aloha_backend::mappers::user_group::insert_user_group;
use aloha_backend::models::user::User;
use aloha_backend::models::user_group::UserGroup;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn insert_user_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    // First create a user group
    let transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    let body = serde_json::json!({
        "username": "test_user",
        "password": "test_password",
        "user_group_id": user_group_result.id
    });
    let mock_server = MockServer::start().await;
    Mock::given(path("/user"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.post_user(&body).await.unwrap();

    assert_eq!(response.username, "test_user");
}

#[tokio::test]
async fn get_all_users_returns_a_200() {
    let app = spawn_app().await;

    // First create a user group
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    // Now create users with the user group ID
    transaction = app.db_pool.begin().await.unwrap();
    let mut users = User::default_vec_test(Some(3));
    for user in &mut users {
        user.user_group_id = Some(user_group_result.id);
    }

    for user in &users {
        insert_user(transaction, user).await.unwrap();
        transaction = app.db_pool.begin().await.unwrap();
    }

    let mock_server = MockServer::start().await;
    Mock::given(path("/users"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.get_all_users().await.unwrap();
    assert!(!response.data.is_empty());
}

#[tokio::test]
async fn get_user_returns_a_200_for_valid_id() {
    let app = spawn_app().await;

    // First create a user group
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    // Now create a user with the user group ID
    transaction = app.db_pool.begin().await.unwrap();
    let mut default_user = User::default_test();
    default_user.user_group_id = Some(user_group_result.id);
    let insert_result = insert_user(transaction, &default_user).await.unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/user/{id}"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.get_user_by_id(insert_result.id).await.unwrap();
    assert_eq!(response.id, insert_result.id);
    assert_eq!(response.username, insert_result.username);
}

#[tokio::test]
async fn update_user_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    // First create a user group
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    // Now create a user with the user group ID
    transaction = app.db_pool.begin().await.unwrap();
    let mut default_user = User::default_test();
    default_user.user_group_id = Some(user_group_result.id);
    let insert_result = insert_user(transaction, &default_user).await.unwrap();

    let body = serde_json::json!({
        "username": "updated_username",
        "password": null,
        "user_group_id": user_group_result.id
    });

    let mock_server = MockServer::start().await;
    Mock::given(path("/user/{id}"))
        .and(method("PUT"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.put_user(insert_result.id, &body).await.unwrap();
    assert_eq!(response.username, "updated_username");
}

// Test for deleting multiple users
#[tokio::test]
async fn delete_users_returns_a_200_for_valid_ids() {
    let app = spawn_app().await;

    // First create a user group
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    // Create first user
    transaction = app.db_pool.begin().await.unwrap();
    let mut user1 = User::default_test();
    user1.user_group_id = Some(user_group_result.id);
    let user1_result = insert_user(transaction, &user1).await.unwrap();

    // Create second user
    transaction = app.db_pool.begin().await.unwrap();
    let mut user2 = User::default_test();
    user2.username = "test_user2".to_string();
    user2.user_group_id = Some(user_group_result.id);
    let user2_result = insert_user(transaction, &user2).await.unwrap();

    let user_ids = vec![user1_result.id, user2_result.id];

    let mock_server = MockServer::start().await;
    Mock::given(path("/user"))
        .and(method("DELETE"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.delete_users(&user_ids).await.unwrap();
    assert_eq!(response.len(), 2);
    assert!(response.iter().any(|u| u.id == user1_result.id));
    assert!(response.iter().any(|u| u.id == user2_result.id));
}

#[tokio::test]
async fn delete_user_returns_a_200_for_valid_id() {
    let app = spawn_app().await;

    // First create a user group
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_group = UserGroup::default_test();
    let user_group_result = insert_user_group(transaction, &user_group).await.unwrap();

    // Now create a user with the user group ID
    transaction = app.db_pool.begin().await.unwrap();
    let mut default_user = User::default_test();
    default_user.user_group_id = Some(user_group_result.id);
    let insert_result = insert_user(transaction, &default_user).await.unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/user/{id}"))
        .and(method("DELETE"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.delete_user(insert_result.id).await.unwrap();
    assert_eq!(response.id, insert_result.id);
}
