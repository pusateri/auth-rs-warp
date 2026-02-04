use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Users {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub email: String,
    pub hash_pass: String,
}
