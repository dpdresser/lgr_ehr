use std::hash::{Hash, Hasher};

use secrecy::{ExposeSecret, SecretString};
use validator::{Validate, ValidateEmail};

use crate::domain::error::app_error::{AppResult, ValidationError};

#[derive(Debug, Clone)]
pub struct Email {
    inner: SecretString,
}

impl AsRef<SecretString> for Email {
    fn as_ref(&self) -> &SecretString {
        &self.inner
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.inner.expose_secret() == other.inner.expose_secret()
    }
}

impl Hash for Email {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.expose_secret().hash(state);
    }
}

impl Eq for Email {}

impl Validate for Email {
    #[tracing::instrument(name = "email_validation", skip_all)]
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        if !self.inner.expose_secret().validate_email() {
            return Err(validator::ValidationErrors::new());
        }

        Ok(())
    }
}

impl Email {
    #[tracing::instrument(name = "email_creation", skip_all)]
    pub fn new(email: String) -> AppResult<Self> {
        let email = Email {
            inner: SecretString::from(email),
        };
        email
            .validate()
            .map_err(|_| ValidationError::InvalidEmail)?;
        Ok(email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_valid_email_creation() {
        let valid_emails = vec![
            "test@example.com",
            "user.name@domain.co.uk",
            "first+last@subdomain.example.org",
            "email123@test-domain.com",
        ];

        for email_str in valid_emails {
            let email = Email::new(email_str.to_string());
            assert!(email.is_ok(), "Should accept valid email: {}", email_str);
        }
    }

    #[test]
    fn test_invalid_email_creation() {
        let invalid_emails = vec![
            "invalid-email",
            "@example.com",
            "test@",
            "test@.com",
            "",
            " ",
            "test @example.com",
        ];

        for email_str in invalid_emails {
            let email = Email::new(email_str.to_string());
            assert!(email.is_err(), "Should reject invalid email: {}", email_str);
        }
    }

    #[test]
    fn test_email_equality() {
        let email1 = Email::new("test@example.com".to_string()).unwrap();
        let email2 = Email::new("test@example.com".to_string()).unwrap();
        let email3 = Email::new("different@example.com".to_string()).unwrap();

        assert_eq!(email1, email2);
        assert_ne!(email1, email3);
    }

    #[test]
    fn test_email_hash() {
        let email1 = Email::new("test@example.com".to_string()).unwrap();
        let email2 = Email::new("test@example.com".to_string()).unwrap();
        let email3 = Email::new("different@example.com".to_string()).unwrap();

        let mut set = HashSet::new();
        set.insert(email1);
        set.insert(email2); // Should not increase size since it's the same email
        set.insert(email3);

        assert_eq!(set.len(), 2, "HashSet should contain 2 unique emails");
    }

    #[test]
    fn test_email_as_ref() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let secret_ref: &SecretString = email.as_ref();

        assert_eq!(secret_ref.expose_secret(), "test@example.com");
    }

    #[test]
    fn test_email_clone() {
        let email1 = Email::new("test@example.com".to_string()).unwrap();
        let email2 = email1.clone();

        assert_eq!(email1, email2);
    }

    #[test]
    fn test_email_debug() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let debug_output = format!("{:?}", email);

        // Debug output should not expose the actual email for security
        assert!(!debug_output.contains("test@example.com"));
    }
}
