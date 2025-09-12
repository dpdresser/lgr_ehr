use crate::domain::types::{email::Email, password::Password};

use secrecy::ExposeSecret;

pub struct User {
    pub user_id: Option<String>,
    pub username: String,
    pub email: Email,
    pub password: Password,
    pub first_name: String,
    pub last_name: String,
    pub role: Option<UserRole>,
}

impl User {
    pub fn new(
        username: String,
        email: Email,
        password: Password,
        first_name: String,
        last_name: String,
        role: Option<UserRole>,
    ) -> Self {
        User {
            user_id: None,
            username,
            email,
            password,
            first_name,
            last_name,
            role,
        }
    }

    pub fn update_user(&mut self, update: UserUpdate) {
        if let Some(user_id) = update.user_id {
            self.user_id = Some(user_id);
        }
        if let Some(email) = update.email {
            self.email = email;
        }
        if let Some(password) = update.password {
            self.password = password;
        }
        if let Some(first_name) = update.first_name {
            self.first_name = first_name;
        }
        if let Some(last_name) = update.last_name {
            self.last_name = last_name;
        }
        if let Some(role) = update.role {
            self.role = Some(role);
        }
    }

    pub fn signup_json(&self, enabled: bool, verified: bool) -> serde_json::Value {
        serde_json::json!({
            "username": self.username,
            "email": self.email.as_ref().expose_secret(),
            "firstName": self.first_name,
            "lastName": self.last_name,
            "enabled": enabled,
            "emailVerified": verified,
            "credentials": [{
                "type": "password",
                "value": self.password.as_ref().expose_secret(),
                "temporary": false
            }],
        })
    }
}

pub struct UserUpdate {
    pub user_id: Option<String>,
    pub email: Option<Email>,
    pub password: Option<Password>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<UserRole>,
}

pub enum UserRole {
    Owner,
    Admin,
    Biller,
    Clinician,
}
