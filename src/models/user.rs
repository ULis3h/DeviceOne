use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
    Viewer,
}

#[derive(Debug, Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<UserRole>,
}

#[derive(Debug, Deserialize)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<UserRole>,
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}
