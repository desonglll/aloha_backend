use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check() {
    let app = spawn_app().await;
    dbg!(&app);
    let client = reqwest::Client::builder()
        .no_proxy()
        .build()
        .expect("Failed to build reqwest");
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
