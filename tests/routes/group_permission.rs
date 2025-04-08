use crate::helpers::spawn_app;
use aloha_backend::mappers::group_permission::insert_group_permission;
use aloha_backend::mappers::permission::insert_permission;
use aloha_backend::mappers::user_group::insert_user_group;
use aloha_backend::models::group_permission::GroupPermission;
use aloha_backend::models::permission::Permission;
use aloha_backend::models::user_group::UserGroup;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn insert_group_permission_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user group and permission first
    let user_group = UserGroup::default_test();
    let permission = Permission::default_test();
    insert_user_group(transaction, &user_group).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    let body = serde_json::json!({
        "group_id": user_group.id,
        "permission_id": permission.id
    });

    let mock_server = MockServer::start().await;
    Mock::given(path("/group-permissions"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.post_group_permission(&body).await.unwrap();

    assert_eq!(response.group_id, user_group.id);
    assert_eq!(response.permission_id, permission.id);
}

#[tokio::test]
async fn get_all_group_permissions_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user group and permission first
    let user_group = UserGroup::default_test();
    let permission = Permission::default_test();
    insert_user_group(transaction, &user_group).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create group permissions using the actual user group and permission IDs
    let group_permission = GroupPermission::new(user_group.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_group_permission(transaction, &group_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/group_permissions"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.get_all_group_permissions().await.unwrap();
    assert!(!response.data.is_empty());
}

#[tokio::test]
async fn get_group_permissions_by_group_id_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user group and permission first
    let user_group = UserGroup::default_test();
    let permission = Permission::default_test();
    insert_user_group(transaction, &user_group).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create group permissions
    let group_permission = GroupPermission::new(user_group.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_group_permission(transaction, &group_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path(
        format!("/group-permissions/group/{}", user_group.id).as_str(),
    ))
    .and(method("GET"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&mock_server)
    .await;

    let response = app
        .get_group_permissions_by_group_id(user_group.id)
        .await
        .unwrap();
    assert!(!response.is_empty());
    assert_eq!(response[0].group_id, user_group.id);
}

#[tokio::test]
async fn get_group_permissions_by_permission_id_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user group and permission first
    let user_group = UserGroup::default_test();
    let permission = Permission::default_test();
    insert_user_group(transaction, &user_group).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create group permissions
    let group_permission = GroupPermission::new(user_group.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_group_permission(transaction, &group_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path(
        format!("/group-permissions/permission/{}", permission.id).as_str(),
    ))
    .and(method("GET"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&mock_server)
    .await;

    let response = app
        .get_group_permissions_by_permission_id(permission.id)
        .await
        .unwrap();
    assert!(!response.is_empty());
    assert_eq!(response[0].permission_id, permission.id);
}

#[tokio::test]
async fn delete_group_permission_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user group and permission first
    let user_group = UserGroup::default_test();
    let permission = Permission::default_test();
    insert_user_group(transaction, &user_group).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create group permission
    let group_permission = GroupPermission::new(user_group.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_group_permission(transaction, &group_permission)
        .await
        .unwrap();

    let body = serde_json::json!({
        "group_id": user_group.id,
        "permission_id": permission.id
    });

    let mock_server = MockServer::start().await;
    Mock::given(path("/group-permissions"))
        .and(method("DELETE"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.delete_group_permission(&body).await.unwrap();
    assert_eq!(response.group_id, user_group.id);
    assert_eq!(response.permission_id, permission.id);
}

#[tokio::test]
async fn delete_group_permissions_by_group_id_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user group and permission first
    let user_group = UserGroup::default_test();
    let permission = Permission::default_test();
    insert_user_group(transaction, &user_group).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create group permission
    let group_permission = GroupPermission::new(user_group.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_group_permission(transaction, &group_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path(
        format!("/group-permissions/group/{}", user_group.id).as_str(),
    ))
    .and(method("DELETE"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&mock_server)
    .await;

    let response = app
        .delete_group_permissions_by_group_id(user_group.id)
        .await
        .unwrap();
    assert!(!response.is_empty());
    assert_eq!(response[0].group_id, user_group.id);
}

#[tokio::test]
async fn delete_group_permissions_by_permission_id_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user group and permission first
    let user_group = UserGroup::default_test();
    let permission = Permission::default_test();
    insert_user_group(transaction, &user_group).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create group permission
    let group_permission = GroupPermission::new(user_group.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_group_permission(transaction, &group_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path(
        format!("/group-permissions/permission/{}", permission.id).as_str(),
    ))
    .and(method("DELETE"))
    .respond_with(ResponseTemplate::new(200))
    .mount(&mock_server)
    .await;

    let response = app
        .delete_group_permissions_by_permission_id(permission.id)
        .await
        .unwrap();
    assert!(!response.is_empty());
    assert_eq!(response[0].permission_id, permission.id);
}
