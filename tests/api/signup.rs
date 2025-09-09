use lgr_ehr::utils::tracing::init_tracing_for_tests;

use crate::helpers::TestApp;

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
            "email": "test@example.com",
            "last_name": "User",
            "password": "password123"
        }),
        // Missing password
        serde_json::json!({
            "email": "test@example.com",
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
            "email": "test@example.com",
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

    let response = app
        .post_signup(serde_json::json!({
            "email": "test@example.com",
            "first_name": "Test",
            "last_name": "User",
            "password": "password123"
        }))
        .await;

    assert_eq!(response.status(), 201);

    app.cleanup().await;
}
