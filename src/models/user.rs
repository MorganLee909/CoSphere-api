use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc, Client, Collection};

#[derive(Deserialize, Serialize)]
pub struct User {
    email: String,
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
    ) -> Self {
        Self {
            email: email,
            password: password,
            first_name: first_name,
            last_name: last_name,
            status: String::from("active"),
            expiration: Utc::now(),
            created_date: Utc::now(),
            reset_code: None,
            avatar: None,
            default_location: String::from("Myrtle Beach"),
            session_id: String::from("12345")
        }
    }

    pub async fn save(&self, client: &Client) {
        let user_collection: Collection<User> = client.database("cosphere").collection("users");
        user_collection.insert_one(self).await;
    }
}

struct StripeData {
    customer_id: String,
    product_id: Option<String>,
    subscription_id: Option<String>,
    subscription_status: Option<String>,
    subscription_type: String
}
