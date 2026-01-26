// -------------------------------
// User & Authentication
// -------------------------------

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Manager,
    Member,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,

    // 外部認証を使う可能性があるので Option として設定
    #[serde(skip)]
    pub password_hash: Option<String>,

    pub role: UserRole,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
