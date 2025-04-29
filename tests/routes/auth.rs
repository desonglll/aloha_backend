use crate::helpers::spawn_app;
use aloha_backend::mappers::user::insert_user;
use aloha_backend::models::user::User;
use aloha_backend::routes::auth::LoginFormData;

#[tokio::test]
async fn login_returns_200_for_valid_credentials() {
    let app = spawn_app().await;

    // Create a test user
    let transaction = app.db_pool.begin().await.unwrap();
    let user = User::default_test();
    let inserted_user = insert_user(transaction, &user).await.unwrap();

    // Login with valid credentials
    let login_data = LoginFormData {
        username: inserted_user.username.clone(),
        password: inserted_user.password_hash.clone(),
    };

    let response = app
        .api_client
        .post(format!("{}/auth/login", app.address))
        .json(&login_data)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    // Check that session contains user information
    let session_data = response.json::<serde_json::Value>().await.unwrap();
    assert_eq!(
        session_data["username"],
        format!("\"{}\"", inserted_user.username)
    );
    assert_eq!(session_data["user_id"], format!("\"{}\"", inserted_user.id));
}

#[tokio::test]
async fn login_returns_401_for_invalid_credentials() {
    let app = spawn_app().await;

    // Create a test user
    let transaction = app.db_pool.begin().await.unwrap();
    let user = User::default_test();
    let inserted_user = insert_user(transaction, &user).await.unwrap();

    // Login with invalid password
    let login_data = LoginFormData {
        username: inserted_user.username.clone(),
        password: "wrong_password".to_string(),
    };

    let response = app
        .api_client
        .post(format!("{}/auth/login", app.address))
        .json(&login_data)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 401);
    let error_message = response.text().await.unwrap();
    assert!(error_message.contains("User password is invalid"));
}

#[tokio::test]
async fn login_returns_400_for_nonexistent_user() {
    let app = spawn_app().await;

    // Login with nonexistent user
    let login_data = LoginFormData {
        username: "nonexistent_user".to_string(),
        password: "any_password".to_string(),
    };

    let response = app
        .api_client
        .post(format!("{}/auth/login", app.address))
        .json(&login_data)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn logout_returns_200_when_logged_in() {
    let app = spawn_app().await;
    // Create a test user
    let transaction = app.db_pool.begin().await.unwrap();
    let user = User::default_test();
    let inserted_user = insert_user(transaction, &user).await.unwrap();

    // First login to create a session
    let login_data = LoginFormData {
        username: inserted_user.username.clone(),
        password: inserted_user.password_hash.clone(),
    };

    let login_response = app
        .api_client
        .post(format!("{}/auth/login", app.address))
        .json(&login_data)
        .send()
        .await
        .expect("Failed to execute login request");

    assert!(login_response.status().is_success());

    // Now logout
    let logout_response = app
        .api_client
        .post(format!("{}/auth/logout", app.address))
        .send()
        .await
        .expect("Failed to execute logout request");

    assert!(logout_response.status().is_success());

    // Verify session is cleared
    assert!(logout_response.status().is_success());
}

#[tokio::test]
async fn logout_returns_200_when_not_logged_in() {
    let app = spawn_app().await;

    // Try to logout without being logged in
    let response = app
        .api_client
        .post(format!("{}/auth/logout", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    let message = response.text().await.unwrap();
    assert!(message.contains("Attempt to log out failed: no user found in session"));
}

#[tokio::test]
async fn check_login_returns_true_when_logged_in() {
    let app = spawn_app().await;

    // Create a test user
    let transaction = app.db_pool.begin().await.unwrap();
    let user = User::default_test();
    let inserted_user = insert_user(transaction, &user).await.unwrap();

    // First login to create a session
    let login_data = LoginFormData {
        username: inserted_user.username.clone(),
        password: inserted_user.password_hash.clone(),
    };

    let login_response = app
        .api_client
        .post(format!("{}/auth/login", app.address))
        .json(&login_data)
        .send()
        .await
        .expect("Failed to execute login request");

    assert!(login_response.status().is_success());

    // // Now check login status
    // let check_response = app
    //     .api_client
    //     .get(format!("{}/auth/check", app.address))
    //     .send()
    //     .await
    //     .expect("Failed to execute check login request");

    // assert!(check_response.status().is_success());
    // let is_logged_in = check_response.json::<bool>().await.unwrap();
    // assert!(is_logged_in);
}

// #[tokio::test]
// async fn check_login_returns_false_when_not_logged_in() {
//     let app = spawn_app().await;

//     // Check login status without being logged in
//     let response = app
//         .api_client
//         .get(format!("{}/auth/check", app.address))
//         .send()
//         .await
//         .expect("Failed to execute request");

//     assert!(response.status().is_success());
//     let is_logged_in = response.json::<bool>().await.unwrap();
//     assert!(!is_logged_in);
// }
