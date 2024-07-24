use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::Regex;
use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier,
    password_hash::{rand_core::OsRng, SaltString}
};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub email: String,
    password: String,
    first_name: String,
    last_name: String,
    status: String,
    expiration: DateTime<Utc>,
    created_date: DateTime<Utc>,
    stripe: Option<StripeData>,
    reset_code: Option<String>,
    avatar: Option<String>,
    default_location: String,
    session_id: String
}

impl User {
    pub async fn new(
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
            stripe: None,
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
        //user.stripe = Some(user.create_stripe_data().await);

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

    async fn create_stripe_data(&self) -> StripeData {
        let stripe_key = std::env::var("COSPHERE_STRIPE_KEY").unwrap();
        let customer: Value = reqwest::Client::new()
            .post("https://api.stripe.com/v1/customers")
            .header("Authorization", format!("Basic {}", stripe_key))
            .body(format!("{{email: {}}}", self.email))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        StripeData {
            customer_id: String::from("Some id"),
            product_id: None,
            subscription_id: None,
            subscription_status: None,
            subscription_type: None
        }
    }

    pub fn valid_password(&self, password: String) -> bool {
        let parsed_hash = PasswordHash::new(&self.password).unwrap();
        Argon2::default().verify_password(&password.into_bytes(), &parsed_hash).is_ok()
    }
}

#[derive(Deserialize, Serialize)]
struct StripeData {
    customer_id: String,
    product_id: Option<String>,
    subscription_id: Option<String>,
    subscription_status: Option<String>,
    subscription_type: Option<String>
}
