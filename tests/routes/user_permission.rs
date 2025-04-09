use crate::helpers::spawn_app;
use aloha_backend::mappers::permission::insert_permission;
use aloha_backend::mappers::user::insert_user;
use aloha_backend::mappers::user_permission::insert_user_permission;
use aloha_backend::models::permission::Permission;
use aloha_backend::models::user::User;
use aloha_backend::models::user_permission::UserPermission;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn insert_user_permission_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user and permission first
    let user = User::default_test();
    let permission = Permission::default_test();
    insert_user(transaction, &user).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    let body = serde_json::json!({
        "user_id": user.id,
        "permission_id": permission.id
    });

    let mock_server = MockServer::start().await;
    Mock::given(path("/user_permissions"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.post_user_permission(&body).await.unwrap();

    assert_eq!(response.user_id, user.id);
    assert_eq!(response.permission_id, permission.id);
}

#[tokio::test]
async fn get_all_user_permissions_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user and permission first
    let user = User::default_test();
    let permission = Permission::default_test();
    insert_user(transaction, &user).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create user permissions using the actual user and permission IDs
    let user_permission = UserPermission::new(user.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_user_permission(transaction, &user_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/user_permissions"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.get_all_user_permissions().await.unwrap();
    assert!(!response.data.is_empty());
}

#[tokio::test]
async fn get_user_permissions_by_user_id_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user and permission first
    let user = User::default_test();
    let permission = Permission::default_test();
    insert_user(transaction, &user).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create user permissions
    let user_permission = UserPermission::new(user.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_user_permission(transaction, &user_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path(format!("/user_permissions/user/{}", user.id).as_str()))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.get_user_permissions_by_user_id(user.id).await.unwrap();
    assert!(!response.data.is_empty());
    assert_eq!(response.data[0].user_id, user.id);
}

#[tokio::test]
async fn get_user_permissions_by_permission_id_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user and permission first
    let user = User::default_test();
    let permission = Permission::default_test();
    insert_user(transaction, &user).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create user permissions
    let user_permission = UserPermission::new(user.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_user_permission(transaction, &user_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path(
        format!("/user_permissions/permission/{}", permission.id).as_str(),
    ))
    .and(method("GET"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&mock_server)
    .await;

    let response = app
        .get_user_permissions_by_permission_id(permission.id)
        .await
        .unwrap();
    assert!(!response.data.is_empty());
    assert_eq!(response.data[0].permission_id, permission.id);
}

#[tokio::test]
async fn delete_user_permission_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user and permission first
    let user = User::default_test();
    let permission = Permission::default_test();
    insert_user(transaction, &user).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create user permission
    let user_permission = UserPermission::new(user.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_user_permission(transaction, &user_permission)
        .await
        .unwrap();

    let body = serde_json::json!({
        "user_id": user.id,
        "permission_id": permission.id
    });

    let mock_server = MockServer::start().await;
    Mock::given(path("/user_permissions"))
        .and(method("DELETE"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.delete_user_permission(&body).await.unwrap();
    assert_eq!(response.user_id, user.id);
    assert_eq!(response.permission_id, permission.id);
}

#[tokio::test]
async fn delete_user_permissions_by_user_id_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user and permission first
    let user = User::default_test();
    let permission = Permission::default_test();
    insert_user(transaction, &user).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create user permission
    let user_permission = UserPermission::new(user.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_user_permission(transaction, &user_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path(format!("/user_permissions/user/{}", user.id).as_str()))
        .and(method("DELETE"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app
        .delete_user_permissions_by_user_id(user.id)
        .await
        .unwrap();
    assert!(!response.data.is_empty());
    assert_eq!(response.data[0].user_id, user.id);
}

#[tokio::test]
async fn delete_user_permissions_by_permission_id_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user and permission first
    let user = User::default_test();
    let permission = Permission::default_test();
    insert_user(transaction, &user).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create user permission
    let user_permission = UserPermission::new(user.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_user_permission(transaction, &user_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path(
        format!("/user_permissions/permission/{}", permission.id).as_str(),
    ))
    .and(method("DELETE"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&mock_server)
    .await;

    let response = app
        .delete_user_permissions_by_permission_id(permission.id)
        .await
        .unwrap();
    assert!(!response.data.is_empty());
    assert_eq!(response.data[0].permission_id, permission.id);
}
