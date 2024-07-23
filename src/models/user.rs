use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use regex::Regex;
use argon2::{Argon2, PasswordHasher, password_hash::{rand_core::OsRng, Salt, SaltString}};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub email: String,
    password: String,
    first_name: String,
    last_name: String,
    status: String,
    expiration: DateTime<Utc>,
    created_date: DateTime<Utc>,
    //pub stripe: StripeData,
    reset_code: Option<String>,
    avatar: Option<String>,
    default_location: String,
    session_id: String
}

impl User {
    pub fn new(
        email: String,
        password: String,
        confirm_password: String,
        first_name: String,
        last_name: String
    ) -> Result<User, (i16, String)>{
        let mut user = Self {
            email: email.to_lowercase(),
            password: password,
            first_name: first_name,
            last_name: last_name,
            status: String::from("active"),
            expiration: Utc::now(),
            created_date: Utc::now(),
            reset_code: None,
            avatar: None,
            default_location: String::from("Some Place"),
            session_id: String::from("12345")
        };

        if !user.passwords_match(confirm_password) {
           return Err((400, String::from("Passwords do not match")));
        }

        if !user.password_valid_length() {
            return Err((400, String::from("Password must contain at least 10 characaters")));
        }

        if !user.email_valid() {
            return Err((400, String::from("Invalid email")));
        }

        user.password = user.hashed_password();

        Ok(user)
    }

    fn passwords_match(&self, password: String) -> bool {
        self.password == password
    }

    fn password_valid_length(&self) -> bool {
        self.password.chars().count() >= 10
    }

    fn email_valid(&self) -> bool {
        let regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        regex.is_match(&self.email)
    }

    fn hashed_password(&self) -> String {
        let salt_str = SaltString::generate(&mut OsRng);
        let salt = salt_str.as_salt();

        let argon2 = Argon2::default();
        argon2.hash_password(self.password.as_bytes(), salt).unwrap().to_string()
    }
}

struct StripeData {
    customer_id: String,
    product_id: Option<String>,
    subscription_id: Option<String>,
    subscription_status: Option<String>,
    subscription_type: String
}
