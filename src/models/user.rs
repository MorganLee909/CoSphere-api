use serde::{Deserialize, Serialize};
use chrono::prelude::{DateTime, Utc};

pub struct User {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    status: String,
    expiration: DateTime<Utc>,
    created_date: DateTime<Utc>,
    stripe: StripeData,
    reset_code: Option<String>,
    avatar: Option<String>,
    default_location: String,
    session_id: String
}

struct StripeData {
    customer_id: String,
    product_id: Option<String>,
    subscription_id: Option<String>,
    subscription_status: Option<String>,
    subscription_type: String
}
