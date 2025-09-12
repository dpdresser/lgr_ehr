use lgr_ehr::utils::tracing::init_tracing_for_tests;

use crate::helpers::{TestApp, generate_valid_email};

#[tokio::test]
async fn get_user_id_should_return_200() {
    init_tracing_for_tests();
    let mut app = TestApp::new().await;

    let email = generate_valid_email();

    let response = app
        .post_signup(serde_json::json!({
            "email": email,
            "first_name": "Test",
            "last_name": "User",
            "password": "Password123!"
        }))
        .await;

    assert_eq!(response.status(), 201);

    let response = app
        .post_get_user_id(serde_json::json!({
            "email": email,
        }))
        .await;

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert!(
        body.get("user_id").is_some(),
        "Response body does not contain user_id"
    );

    let response = app.post_delete_user(body).await;

    assert_eq!(response.status(), 200);

    app.cleanup().await;
}
