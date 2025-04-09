use aloha_backend::dto::query::DtoQuery;
use aloha_backend::mappers::user::*;
use aloha_backend::models::user::User;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use uuid::Uuid;

async fn setup_test_db() -> Result<PgPool, sqlx::Error> {
    // Try to get DATABASE_URL from environment, or use a default test database URL
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        // Use a default test database URL - make sure this exists in your test environment
        "postgres://postgres:password@localhost:5432/aloha_test".to_string()
    });

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Clean up existing data - use a transaction to ensure atomicity
    let mut tx = pool.begin().await?;
    sqlx::query!("DELETE FROM users").execute(&mut *tx).await?;
    tx.commit().await?;

    Ok(pool)
}

#[tokio::test]
async fn test_user_crud_operations() {
    let pool = match setup_test_db().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Skipping test, database connection failed: {}", e);
            return;
        }
    };

    // Test insert
    let user = User {
        id: Uuid::new_v4(),
        username: "test_user".to_string(),
        password_hash: "hashed_password".to_string(),
        created_at: None,
        user_group_id: None,
    };

    let transaction = pool.begin().await.expect("Failed to begin transaction");
    let inserted_user = insert_user(transaction, &user)
        .await
        .expect("Failed to insert user");

    assert_eq!(inserted_user.username, user.username);
    assert_eq!(inserted_user.password_hash, user.password_hash);
    assert_eq!(inserted_user.id, user.id);

    // Test get by id
    let transaction = pool.begin().await.expect("Failed to begin transaction");
    let fetched_user = get_user_by_id(transaction, user.id)
        .await
        .expect("Failed to get user by id");
    assert_eq!(fetched_user.clone().unwrap().id, user.id);
    assert_eq!(fetched_user.clone().unwrap().username, user.username);

    // Test get by username
    let transaction = pool.begin().await.expect("Failed to begin transaction");
    let fetched_by_username = get_user_by_username(transaction, &user.username)
        .await
        .expect("Failed to get user by username");
    assert_eq!(fetched_by_username.id, user.id);
    assert_eq!(fetched_by_username.username, user.username);

    // Test update
    let transaction = pool.begin().await.expect("Failed to begin transaction");
    let updated_user = User {
        username: "updated_username".to_string(),
        ..user.clone()
    };
    let updated = update_user(transaction, &updated_user)
        .await
        .expect("Failed to update user");
    assert_eq!(updated.username, "updated_username");

    // Test delete
    let transaction = pool.begin().await.expect("Failed to begin transaction");
    let deleted_user = delete_user_by_id(transaction, user.id)
        .await
        .expect("Failed to delete user");
    assert_eq!(deleted_user.id, user.id);
}

#[tokio::test]
async fn test_get_all_users() {
    let pool = match setup_test_db().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Skipping test, database connection failed: {}", e);
            return;
        }
    };

    // Insert multiple users
    let users = vec![
        User {
            id: Uuid::new_v4(),
            username: "user1".to_string(),
            password_hash: "hash1".to_string(),
            created_at: None,
            user_group_id: None,
        },
        User {
            id: Uuid::new_v4(),
            username: "user2".to_string(),
            password_hash: "hash2".to_string(),
            created_at: None,
            user_group_id: None,
        },
    ];

    for user in &users {
        let transaction = pool.begin().await.expect("Failed to begin transaction");
        insert_user(transaction, user)
            .await
            .expect("Failed to insert test user");
    }

    // Test pagination
    let transaction = pool.begin().await.expect("Failed to begin transaction");
    let dto_query = DtoQuery::default_query();
    let response = get_all_users(transaction, dto_query)
        .await
        .expect("Failed to get all users");

    assert_eq!(response.data.len(), 2);
    assert!(response.pagination.is_some());
    if let Some(pagination) = response.pagination {
        assert_eq!(pagination.total.unwrap(), 2);
    }
}

#[tokio::test]
async fn test_delete_users_by_ids() {
    let pool = match setup_test_db().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Skipping test, database connection failed: {}", e);
            return;
        }
    };

    // Create test users
    let users = vec![
        User {
            id: Uuid::new_v4(),
            username: "bulk_user1".to_string(),
            password_hash: "hash1".to_string(),
            created_at: None,
            user_group_id: None,
        },
        User {
            id: Uuid::new_v4(),
            username: "bulk_user2".to_string(),
            password_hash: "hash2".to_string(),
            created_at: None,
            user_group_id: None,
        },
        User {
            id: Uuid::new_v4(),
            username: "bulk_user3".to_string(),
            password_hash: "hash3".to_string(),
            created_at: None,
            user_group_id: None,
        },
    ];

    // Insert users
    for user in &users {
        let transaction = pool.begin().await.expect("Failed to begin transaction");
        insert_user(transaction, user)
            .await
            .expect("Failed to insert test user");
    }

    // Get IDs for bulk delete
    let ids_to_delete: Vec<Uuid> = users.iter().take(2).map(|u| u.id).collect();

    // Test bulk delete
    let transaction = pool.begin().await.expect("Failed to begin transaction");
    let deleted_users = delete_users_by_ids(transaction, ids_to_delete.clone())
        .await
        .expect("Failed to delete multiple users");

    assert_eq!(deleted_users.len(), 2);
    assert!(deleted_users.iter().all(|u| ids_to_delete.contains(&u.id)));

    // Verify remaining user
    let transaction = pool.begin().await.expect("Failed to begin transaction");
    let remaining_user = get_user_by_id(transaction, users[2].id)
        .await
        .expect("Failed to get remaining user");
    assert_eq!(remaining_user.unwrap().username, "bulk_user3");
}
