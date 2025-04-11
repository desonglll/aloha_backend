use aloha_backend::dto::query::DtoQuery;
use aloha_backend::mappers::permission::insert_permission;
use aloha_backend::mappers::user::insert_user;
use aloha_backend::mappers::user_permission::{
    delete_user_permission, delete_user_permissions_by_permission_id,
    delete_user_permissions_by_user_id, get_all_user_permissions,
    get_user_permissions_by_permission_id, get_user_permissions_by_user_id, insert_user_permission,
};
use aloha_backend::models::permission::Permission;
use aloha_backend::models::user::User;
use aloha_backend::models::user_permission::UserPermission;

#[tokio::test]
async fn insert_user_permission_works() {
    let app = crate::helpers::spawn_app().await;
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
    let result = insert_user_permission(transaction, &user_permission)
        .await
        .unwrap();

    assert_eq!(result.user_id, user.id);
    assert_eq!(result.permission_id, permission.id);
}

#[tokio::test]
async fn get_all_user_permissions_works() {
    let app = crate::helpers::spawn_app().await;
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

    // Get all user permissions
    transaction = app.db_pool.begin().await.unwrap();
    let result = get_all_user_permissions(transaction, DtoQuery::default_query())
        .await
        .unwrap();

    assert!(!result.data.is_empty());
    let found = result
        .data
        .iter()
        .any(|up| up.user_id == user.id && up.permission_id == permission.id);
    assert!(found);
}

#[tokio::test]
async fn get_user_permissions_by_user_id_works() {
    let app = crate::helpers::spawn_app().await;
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

    // Get user permissions by user_id
    transaction = app.db_pool.begin().await.unwrap();
    let result = get_user_permissions_by_user_id(transaction, user.id)
        .await
        .unwrap();

    assert!(!result.is_empty());
    assert_eq!(result[0].user_id, user.id);
    assert_eq!(result[0].permission_id, permission.id);
}

#[tokio::test]
async fn get_user_permissions_by_permission_id_works() {
    let app = crate::helpers::spawn_app().await;
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

    // Get user permissions by permission_id
    transaction = app.db_pool.begin().await.unwrap();
    let result = get_user_permissions_by_permission_id(transaction, permission.id)
        .await
        .unwrap();

    assert!(!result.is_empty());
    assert_eq!(result[0].user_id, user.id);
    assert_eq!(result[0].permission_id, permission.id);
}

#[tokio::test]
async fn delete_user_permission_works() {
    let app = crate::helpers::spawn_app().await;
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

    // Delete user permission
    transaction = app.db_pool.begin().await.unwrap();
    let result = delete_user_permission(transaction, user.id, permission.id)
        .await
        .unwrap();

    assert_eq!(result.user_id, user.id);
    assert_eq!(result.permission_id, permission.id);

    // Verify deletion
    transaction = app.db_pool.begin().await.unwrap();
    let permissions = get_user_permissions_by_user_id(transaction, user.id)
        .await
        .unwrap();
    assert!(permissions.is_empty());
}

#[tokio::test]
async fn delete_user_permissions_by_user_id_works() {
    let app = crate::helpers::spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create a user and permissions
    let mut user = User::default_test();
    // Make username unique to avoid constraint violation
    user.username = format!("user_{}", uuid::Uuid::new_v4());

    let mut permission1 = Permission::default_test();
    // Make permission names unique to avoid constraint violation
    permission1.name = format!("permission_{}", uuid::Uuid::new_v4());

    let mut permission2 = Permission::default_test();
    permission2.name = format!("permission_{}", uuid::Uuid::new_v4());

    insert_user(transaction, &user).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission1).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission2).await.unwrap();

    // Create user permissions
    let user_permission1 = UserPermission::new(user.id, permission1.id);
    let user_permission2 = UserPermission::new(user.id, permission2.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_user_permission(transaction, &user_permission1)
        .await
        .unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_user_permission(transaction, &user_permission2)
        .await
        .unwrap();

    // Delete user permissions by user_id
    transaction = app.db_pool.begin().await.unwrap();
    let result = delete_user_permissions_by_user_id(transaction, user.id)
        .await
        .unwrap();

    assert_eq!(result.len(), 2);
    assert!(result.iter().any(|up| up.permission_id == permission1.id));
    assert!(result.iter().any(|up| up.permission_id == permission2.id));

    // Verify deletion
    transaction = app.db_pool.begin().await.unwrap();
    let permissions = get_user_permissions_by_user_id(transaction, user.id)
        .await
        .unwrap();
    assert!(permissions.is_empty());
}

#[tokio::test]
async fn delete_user_permissions_by_permission_id_works() {
    let app = crate::helpers::spawn_app().await;
    let mut transaction = app.db_pool.begin().await.unwrap();

    // Create users and a permission
    let mut user1 = User::default_test();
    // Make usernames unique to avoid constraint violation
    user1.username = format!("user_{}", uuid::Uuid::new_v4());

    let mut user2 = User::default_test();
    user2.username = format!("user_{}", uuid::Uuid::new_v4());

    let mut permission = Permission::default_test();
    // Make permission name unique to avoid constraint violation
    permission.name = format!("permission_{}", uuid::Uuid::new_v4());

    insert_user(transaction, &user1).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_user(transaction, &user2).await.unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_permission(transaction, &permission).await.unwrap();

    // Create user permissions
    let user_permission1 = UserPermission::new(user1.id, permission.id);
    let user_permission2 = UserPermission::new(user2.id, permission.id);
    transaction = app.db_pool.begin().await.unwrap();
    insert_user_permission(transaction, &user_permission1)
        .await
        .unwrap();
    transaction = app.db_pool.begin().await.unwrap();
    insert_user_permission(transaction, &user_permission2)
        .await
        .unwrap();

    // Delete user permissions by permission_id
    transaction = app.db_pool.begin().await.unwrap();
    let result = delete_user_permissions_by_permission_id(transaction, permission.id)
        .await
        .unwrap();

    assert_eq!(result.len(), 2);
    assert!(result.iter().any(|up| up.user_id == user1.id));
    assert!(result.iter().any(|up| up.user_id == user2.id));

    // Verify deletion
    transaction = app.db_pool.begin().await.unwrap();
    let permissions = get_user_permissions_by_permission_id(transaction, permission.id)
        .await
        .unwrap();
    assert!(permissions.is_empty());
}
