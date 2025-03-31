use crate::helpers::spawn_app;
use aloha_backend::mappers::user_group::get_group_by_id;
use aloha_backend::models::user_group::UserGroup;
use anyhow::Context;
use sqlx::Executor;

#[tokio::test]
async fn test_get_user_group_by_id() {
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
