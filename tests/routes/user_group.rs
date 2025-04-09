use crate::helpers::spawn_app;
use aloha_backend::mappers::user_group::insert_user_group;
use aloha_backend::models::user_group::UserGroup;
use aloha_backend::routes::user_group::PutUserGroupFormData;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn insert_user_group_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = serde_json::json!({"group_name": "Default Group"});
    let mock_server = MockServer::start().await;
    Mock::given(path("/user_group"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.post_user_group(&body).await.unwrap();

    assert_eq!(response.group_name, "Default Group");
}

#[tokio::test]
async fn get_all_user_group_returns_a_200() {
    let app = spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();
    let user_groups = UserGroup::default_vec_test(Some(3));
    for user_group in &user_groups {
        insert_user_group(transaction, user_group).await.unwrap();
        transaction = app.db_pool.begin().await.unwrap();
    }

    let mock_server = MockServer::start().await;
    Mock::given(path("/user_groups"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.get_all_user_groups().await.unwrap();
    assert!(!response.data.is_empty());
}

#[tokio::test]
async fn get_user_group_returns_a_200_for_valid_id() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();
    let default_user_group = UserGroup::default_test();
    let insert_result = insert_user_group(transaction, &default_user_group)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/user_group/{id}"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.get_user_group_by_id(insert_result.id).await.unwrap();
    assert_eq!(response, insert_result.into());
}

#[tokio::test]
async fn update_user_group_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();
    let default_user_group = UserGroup::default_test();
    let insert_result = insert_user_group(transaction, &default_user_group)
        .await
        .unwrap();

    let mock_server = MockServer::start().await;
    Mock::given(path("/user_groups"))
        .and(method("PUT"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let mut update = insert_result.clone();
    update.group_name = String::from("Updated Group");

    let update_user_group = PutUserGroupFormData {
        id: insert_result.id,
        group_name: String::from("Updated Group"),
    };
    let json_value = serde_json::to_value(&update_user_group).unwrap();
    dbg!(&json_value);
    let response = app.put_user_group(&json_value).await.unwrap();
    assert_eq!(response, update.into());
}

#[tokio::test]
async fn delete_user_group_returns_a_200_for_valid_id() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();
    let default_user_group = UserGroup::default_test();
    let insert_result = insert_user_group(transaction, &default_user_group)
        .await
        .unwrap();
    let mock_server = MockServer::start().await;
    Mock::given(path("/user_group/{id}"))
        .and(method("DELETE"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.delete_user_group(insert_result.id).await.unwrap();
    assert_eq!(response, insert_result.into());
}
