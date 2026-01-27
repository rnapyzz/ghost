// -------------------------------
// User & Authentication
// -------------------------------

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    Admin,
    Manager,
    Member,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
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

impl User {
    pub fn new(name: String, email: String, password_hash: String) -> anyhow::Result<Self> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Name cannot be empty"));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            email,
            password_hash: Some(password_hash),
            role: UserRole::Member,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    // SSO用のnew関数ではpassword_hashが不要
    // pub fn new_sso(name: String, email: String) -> anyhow::Result<Self> {}

    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> anyhow::Result<User>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<User>>;
}
