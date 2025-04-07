use crate::helpers::spawn_app;
use aloha_backend::mappers::user_group::{
    delete_user_group_by_id, get_all_groups, get_group_by_id, insert_user_group, update_user_group,
};
use aloha_backend::models::user_group::UserGroup;
use anyhow::Context;

#[tokio::test]
async fn test_get_all_user_groups_success() {
    let app = spawn_app().await;

    let mut transaction = app
        .db_pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")
        .unwrap();

    let user_groups = UserGroup::default_vec_test(Some(3));
    for user_group in &user_groups {
        sqlx::query!(
            "insert into user_groups (id, group_name) values ($1, $2)",
            user_group.id.clone(),
            user_group.group_name
        )
        .execute(&mut *transaction)
        .await
        .unwrap();
    }
    let result = get_all_groups(transaction).await.unwrap();
    assert_eq!(result.len(), user_groups.len());
}
#[tokio::test]
async fn test_get_user_group_by_id_success() {
    let app = spawn_app().await;

    let mut transaction = app
        .db_pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")
        .unwrap();

    let user_group = UserGroup::default_test();
    sqlx::query!(
        "insert into user_groups (id, group_name) values ($1, $2)",
        user_group.id.clone(),
        user_group.group_name
    )
    .execute(&mut *transaction)
    .await
    .unwrap();
    let result = get_group_by_id(transaction, user_group.id).await.unwrap();
    assert_eq!(result.id.clone(), user_group.id);
}

#[tokio::test]
async fn test_insert_user_group_success() {
    let app = spawn_app().await;
    let user_group = UserGroup::default_test();
    let transaction = app.db_pool.begin().await.unwrap();
    let result = insert_user_group(transaction, &user_group).await.unwrap();
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
    let user_group = UserGroup::default_test();
    let inserted_user_group = sqlx::query_as!(
        UserGroup,
        "insert into user_groups (id, group_name) values ($1, $2) returning id, group_name",
        user_group.id.clone(),
        user_group.group_name
    )
    .fetch_one(&mut *transaction)
    .await
    .unwrap();

    assert_eq!(inserted_user_group, user_group);

    let result = delete_user_group_by_id(transaction, user_group.id)
        .await
        .unwrap();
    assert_eq!(result, user_group);
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
    let user_group = UserGroup::default_test();
    sqlx::query!(
        "insert into user_groups (id, group_name) values ($1, $2)",
        user_group.id.clone(),
        user_group.group_name
    )
    .execute(&mut *transaction)
    .await
    .unwrap();

    let mut updated_user_group = user_group.clone();
    updated_user_group.group_name = String::from("Updated User Group");
    assert_eq!(updated_user_group.id, user_group.id);

    let update_result = update_user_group(transaction, &updated_user_group)
        .await
        .unwrap();
    assert_eq!(updated_user_group, update_result);
}
