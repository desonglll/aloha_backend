use crate::helpers::spawn_app;
use aloha_backend::mappers::permission::insert_permission;
use aloha_backend::models::permission::Permission;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn insert_permission_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name": "Default Permission",
        "description": "Default permission description"
    });
    let mock_server = MockServer::start().await;
    Mock::given(path("/permission"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.post_permission(&body).await.unwrap();

    assert_eq!(response.name, "Default Permission");
    assert_eq!(
        response.description,
        Some("Default permission description".to_string())
    );
}

#[tokio::test]
async fn get_all_permissions_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();
    let permissions = Permission::default_vec_test(Some(3));
    for permission in &permissions {
        insert_permission(transaction, permission).await.unwrap();
        transaction = app.db_pool.begin().await.unwrap();
    }

    let mock_server = MockServer::start().await;
    Mock::given(path("/permissions"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.get_all_permissions().await.unwrap();
    assert!(!response.data.is_empty());
}

#[tokio::test]
async fn get_permission_returns_a_200_for_valid_id() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();
    let default_permission = Permission::default_test();
    let insert_result = insert_permission(transaction, &default_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/permission/{id}"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.get_permission_by_id(insert_result.id).await.unwrap();
    assert_eq!(response.id, insert_result.id);
    assert_eq!(response.name, insert_result.name);
}

#[tokio::test]
async fn update_permission_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();
    let default_permission = Permission::default_test();
    let insert_result = insert_permission(transaction, &default_permission)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/permission"))
        .and(method("PUT"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let mut update = insert_result.clone();
    update.name = String::from("Updated Permission");
    update.description = Some(String::from("Updated description"));
    let json_value = serde_json::to_value(&update).unwrap();
    let response = app.put_permission(&json_value).await.unwrap();
    assert_eq!(response.name, "Updated Permission");
    assert_eq!(
        response.description,
        Some("Updated description".to_string())
    );
}

#[tokio::test]
async fn delete_permission_returns_a_200_for_valid_id() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();
    let default_permission = Permission::default_test();
    let insert_result = insert_permission(transaction, &default_permission)
        .await
        .unwrap();
    let mock_server = MockServer::start().await;
    Mock::given(path("/permission/{id}"))
        .and(method("DELETE"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.delete_permission(insert_result.id).await.unwrap();
    assert_eq!(response.id, insert_result.id);
}
