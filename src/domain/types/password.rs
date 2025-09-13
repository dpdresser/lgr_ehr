use std::hash::{Hash, Hasher};

use secrecy::{ExposeSecret, SecretString};
use validator::Validate;

use crate::domain::error::app_error::{AppResult, ValidationError};

#[derive(Debug, Clone)]
pub struct Password {
    inner: SecretString,
}

impl AsRef<SecretString> for Password {
    fn as_ref(&self) -> &SecretString {
        &self.inner
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.inner.expose_secret() == other.inner.expose_secret()
    }
}

impl Hash for Password {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.expose_secret().hash(state);
    }
}

impl Eq for Password {}

impl Validate for Password {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        let password = self.inner.expose_secret();

        // Password must be at least 8 characters long
        if password.len() < 8 {
            return Err(validator::ValidationErrors::new());
        }

        // Must contain at least 1 number
        if !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(validator::ValidationErrors::new());
        }

        // Must contain at least 1 special character (non-alphanumeric)
        if !password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(validator::ValidationErrors::new());
        }

        Ok(())
    }
}

impl Password {
    pub fn new(password: String) -> AppResult<Self> {
        let password = Password {
            inner: SecretString::from(password),
        };
        password
            .validate()
            .map_err(|_| ValidationError::InvalidPassword)?;
        Ok(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_valid_password_creation() {
        let valid_passwords = vec![
            "Password1!",
            "MySecure123@",
            "Complex9#Pass",
            "Test1234$",
            "Strong5%Word",
            "Abc123!@#",
        ];

        for password_str in valid_passwords {
            let password = Password::new(password_str.to_string());
            assert!(
                password.is_ok(),
                "Should accept valid password: {}",
                password_str
            );
        }
    }

    #[test]
    fn test_password_too_short() {
        let short_passwords = vec![
            "Pass1!", // 6 chars
            "Abc1@",  // 5 chars
            "1!",     // 2 chars
            "",       // empty
        ];

        for password_str in short_passwords {
            let password = Password::new(password_str.to_string());
            assert!(
                password.is_err(),
                "Should reject short password: {}",
                password_str
            );
        }
    }

    #[test]
    fn test_password_no_digit() {
        let no_digit_passwords = vec!["Password!", "MySecure@Test", "Complex#Pass", "NoNumbers!@#"];

        for password_str in no_digit_passwords {
            let password = Password::new(password_str.to_string());
            assert!(
                password.is_err(),
                "Should reject password without digit: {}",
                password_str
            );
        }
    }

    #[test]
    fn test_password_no_special_character() {
        let no_special_passwords = vec![
            "Password123",
            "MySecure123Test",
            "Complex9Pass",
            "OnlyAlphaNum123",
        ];

        for password_str in no_special_passwords {
            let password = Password::new(password_str.to_string());
            assert!(
                password.is_err(),
                "Should reject password without special character: {}",
                password_str
            );
        }
    }

    #[test]
    fn test_password_with_various_special_characters() {
        let special_char_passwords = vec![
            "Password1!",
            "Test123@",
            "Pass9#word",
            "Secure5$pass",
            "Valid8%word", // Added more characters to make it longer
            "Good2^test",
            "Nice7&word",
            "Cool3*pass",
            "Best1+word",
            "Top4=pass",
            "New6|word",
            "Big8\\test",
            "Fast2/word",
            "High5?pass",
            "Low9<word",
            "Mid3>test",
            "End1~pass",
            "Start7`word",
        ];

        for password_str in special_char_passwords {
            let password = Password::new(password_str.to_string());
            assert!(
                password.is_ok(),
                "Should accept password with special character: {}",
                password_str
            );
        }
    }

    #[test]
    fn test_password_equality() {
        let password1 = Password::new("Password1!".to_string()).unwrap();
        let password2 = Password::new("Password1!".to_string()).unwrap();
        let password3 = Password::new("Different2@".to_string()).unwrap();

        assert_eq!(password1, password2);
        assert_ne!(password1, password3);
    }

    #[test]
    fn test_password_hash() {
        let password1 = Password::new("Password1!".to_string()).unwrap();
        let password2 = Password::new("Password1!".to_string()).unwrap();
        let password3 = Password::new("Different2@".to_string()).unwrap();

        let mut set = HashSet::new();
        set.insert(password1);
        set.insert(password2); // Should not increase size since it's the same password
        set.insert(password3);

        assert_eq!(set.len(), 2, "HashSet should contain 2 unique passwords");
    }

    #[test]
    fn test_password_as_ref() {
        let password = Password::new("Password1!".to_string()).unwrap();
        let secret_ref: &SecretString = password.as_ref();

        assert_eq!(secret_ref.expose_secret(), "Password1!");
    }

    #[test]
    fn test_password_clone() {
        let password1 = Password::new("Password1!".to_string()).unwrap();
        let password2 = password1.clone();

        assert_eq!(password1, password2);
    }

    #[test]
    fn test_password_debug() {
        let password = Password::new("Password1!".to_string()).unwrap();
        let debug_output = format!("{:?}", password);

        // Debug output should not expose the actual password for security
        assert!(!debug_output.contains("Password1!"));
    }

    #[test]
    fn test_edge_case_exactly_8_characters() {
        let exactly_8_char_passwords = vec![
            "Pass123!", // exactly 8 chars with all requirements
            "Test45@#", // exactly 8 chars with all requirements
            "Word9!Ab", // exactly 8 chars with all requirements
        ];

        for password_str in exactly_8_char_passwords {
            let password = Password::new(password_str.to_string());
            assert!(
                password.is_ok(),
                "Should accept 8-character password: {}",
                password_str
            );
        }
    }

    #[test]
    fn test_password_with_unicode_characters() {
        let unicode_passwords = vec![
            "Password1🔒", // emoji as special character
            "Test123→",    // arrow as special character
            "Pass9♠️",     // card suit as special character
        ];

        for password_str in unicode_passwords {
            let password = Password::new(password_str.to_string());
            assert!(
                password.is_ok(),
                "Should accept password with unicode special characters: {}",
                password_str
            );
        }
    }

    #[test]
    fn test_password_validation_trait() {
        let valid_password = Password::new("Password1!".to_string()).unwrap();
        assert!(valid_password.validate().is_ok());

        // Test invalid password by creating it manually (bypassing constructor validation)
        let invalid_password = Password {
            inner: SecretString::from("short".to_string()),
        };
        assert!(invalid_password.validate().is_err());
    }
}
