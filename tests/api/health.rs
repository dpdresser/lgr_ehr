use crate::helpers::TestApp;

#[tokio::test]
async fn test_health_check() {
    let app = TestApp::new().await;

    let response = app.health_check().await;

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert_eq!(body, "EHR API is running");
}
