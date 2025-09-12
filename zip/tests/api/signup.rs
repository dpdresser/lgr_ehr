use lgr_ehr::utils::tracing::init_tracing_for_tests;

use crate::helpers::{TestApp, generate_valid_email};

#[tokio::test]
async fn signup_should_return_400_for_malformed_request() {
    init_tracing_for_tests();
    let mut app = TestApp::new().await;

    let test_cases = vec![
        // Missing email
        serde_json::json!({
            "first_name": "Test",
            "last_name": "User",
            "password": "password123"
        }),
        // Missing first name
        serde_json::json!({
            "email": generate_valid_email(),
            "last_name": "User",
            "password": "password123"
        }),
        // Missing password
        serde_json::json!({
            "email": generate_valid_email(),
            "first_name": "Test",
            "last_name": "User"
        }),
    ];

    for (i, test_case) in test_cases.into_iter().enumerate() {
        let response = app.post_signup(test_case).await;
        assert_eq!(response.status(), 400, "Test case {} failed", i);
    }

    app.cleanup().await;
}

#[tokio::test]
async fn signup_should_return_400_for_invalid_input() {
    init_tracing_for_tests();
    let mut app = TestApp::new().await;

    let test_cases = vec![
        // Invalid email
        serde_json::json!({
            "email": "not-an-email",
            "first_name": "Test",
            "last_name": "User",
            "password": "password123"
        }),
        // Password too short
        serde_json::json!({
            "email": generate_valid_email(),
            "first_name": "Test",
            "last_name": "User",
            "password": "short"
        }),
    ];

    for (i, test_case) in test_cases.into_iter().enumerate() {
        let response = app.post_signup(test_case).await;
        assert_eq!(response.status(), 400, "Test case {} failed", i);
    }

    app.cleanup().await;
}

#[tokio::test]
async fn signup_should_return_201() {
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

#[tokio::test]
async fn signup_should_return_409_for_duplicate_email() {
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

    let response = app
        .post_signup(serde_json::json!({
            "email": email,
            "first_name": "Test",
            "last_name": "User",
            "password": "Password123!"
        }))
        .await;

    assert_eq!(response.status(), 409);

    let response = app.post_delete_user(body).await;

    assert_eq!(response.status(), 200);

    app.cleanup().await;
}
