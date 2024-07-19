use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub status: String,
    pub expiration: DateTime<Utc>,
    pub created_date: DateTime<Utc>,
    //pub stripe: StripeData,
    pub reset_code: Option<String>,
    pub avatar: Option<String>,
    pub default_location: String,
    pub session_id: String
}

struct StripeData {
    customer_id: String,
    product_id: Option<String>,
    subscription_id: Option<String>,
    subscription_status: Option<String>,
    subscription_type: String
}
