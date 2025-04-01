use crate::helpers::spawn_app;
use aloha_backend::mappers::user_group::insert_user_group;
use aloha_backend::models::user_group::UserGroup;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn insert_user_group_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "group_name=TestGroup";
    let mock_server = MockServer::start().await;
    Mock::given(path("/user_group"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let response = app.post_user_group(body.into()).await;

    assert_eq!(200, response.status().as_u16());
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
    Mock::given(path("/user_group"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    let response = app.get_user_group_by_id(insert_result.id).await.unwrap();
    assert_eq!(response, insert_result);
}
