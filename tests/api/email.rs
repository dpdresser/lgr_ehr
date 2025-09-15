use crate::helpers::{TestApp, generate_valid_email};
use std::time::Duration;

#[tokio::test]
async fn test_email_send_is_200() {
    let mut app = TestApp::new().await;

    let email = generate_valid_email();

    let request_body = serde_json::json!({
        "to": email
    });

    let response = app.post_send_test_email(request_body).await;

    assert_eq!(response.status(), 200);

    let response_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body as JSON");

    assert_eq!(response_body["to"], email);
    assert_eq!(response_body["message"], "Email sent successfully");

    let mailhog_response = app.wait_for_email(&email, Duration::from_secs(10)).await;

    assert!(
        mailhog_response.is_ok(),
        "Email should be received in MailHog"
    );

    let emails = mailhog_response.unwrap();

    assert!(
        emails["total"].as_u64().unwrap_or(0) > 0,
        "At least one email should be found in MailHog"
    );

    if let Some(items) = emails["items"].as_array() {
        if let Some(first_email) = items.first() {
            let to_addresses = &first_email["To"];
            assert!(
                to_addresses
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .any(
                        |addr| addr["Mailbox"].as_str() == Some(email.split('@').next().unwrap())
                            && addr["Domain"].as_str() == Some("example.com")
                    ),
                "Email should be sent to the correct recipient"
            );

            assert_eq!(
                first_email["Content"]["Headers"]["Subject"]
                    .as_array()
                    .and_then(|arr| arr.first())
                    .and_then(|s| s.as_str()),
                Some("Test Email"),
                "Email should have correct subject"
            );

            let body = first_email["Content"]["Body"].as_str().unwrap_or("");
            assert!(
                body.contains("<h1>This is a test email</h1>"),
                "Email should contain the expected HTML content"
            );
        }
    }

    app.cleanup().await;
}
