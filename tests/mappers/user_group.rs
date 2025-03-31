use crate::helpers::spawn_app;
use aloha_backend::mappers::user_group::{
    delete_user_group_by_id, get_group_by_id, insert_user_group, update_user_group,
};
use aloha_backend::models::user_group::UserGroup;
use anyhow::Context;
use sqlx::Executor;

#[tokio::test]
async fn test_get_user_group_by_id_success() {
    let app = spawn_app().await;

    let mut transaction = app
        .db_pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")
        .unwrap();

    let user_group = UserGroup::default();
    dbg!(&user_group);
    let query = sqlx::query!(
        "insert into user_groups (id, group_name) values ($1, $2)",
        user_group.id.clone(),
        user_group.group_name
    );
    transaction.execute(query).await.unwrap();
    transaction.commit().await.unwrap();
    let result = get_group_by_id(&app.db_pool, user_group.id).await.unwrap();
    assert_eq!(result.id.clone(), user_group.id);
}

#[tokio::test]
async fn test_insert_user_group_success() {
    let app = spawn_app().await;
    let user_group = UserGroup::default();
    let result = insert_user_group(&app.db_pool, &user_group).await.unwrap();
    assert_eq!(user_group, result);
}

#[tokio::test]
async fn test_delete_user_group_success() {
    let app = spawn_app().await;
    let mut transaction = app
        .db_pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")
        .unwrap();
    let user_group = UserGroup::default();
    let query = sqlx::query!(
        "insert into user_groups (id, group_name) values ($1, $2)",
        user_group.id.clone(),
        user_group.group_name
    );
    transaction.execute(query).await.unwrap();
    transaction.commit().await.unwrap();
    let result = delete_user_group_by_id(&app.db_pool, user_group.id)
        .await
        .unwrap();
    assert_eq!(result, user_group)
}
#[tokio::test]
async fn test_update_user_group_success() {
    let app = spawn_app().await;
    let mut transaction = app
        .db_pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")
        .unwrap();
    let user_group = UserGroup::default();
    let query = sqlx::query!(
        "insert into user_groups (id, group_name) values ($1, $2)",
        user_group.id.clone(),
        user_group.group_name
    );
    transaction.execute(query).await.unwrap();
    transaction.commit().await.unwrap();

    let mut updated_user_group = user_group.clone();
    updated_user_group.group_name = String::from("Updated User Group");
    assert_eq!(updated_user_group.id, user_group.id);

    let update_result = update_user_group(&app.db_pool, &updated_user_group)
        .await
        .unwrap();
    assert_eq!(updated_user_group, update_result);
}
