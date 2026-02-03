use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub display_order: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Service {
    pub fn new(name: String, slug: String, display_order: i32) -> anyhow::Result<Self> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Name cannot be empty"));
        }

        let re = Regex::new(r"^[a-z0-9-]+$")?;
        if !re.is_match(&slug) {
            return Err(anyhow::anyhow!(
                "Slug must contain only lowercase letters, numbers, and hyphens"
            ));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            slug,
            display_order,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }
}

#[async_trait::async_trait]
pub trait ServiceRepository: Send + Sync {
    async fn create(&self, service: &Service) -> anyhow::Result<Service>;
    async fn find_all(&self) -> anyhow::Result<Vec<Service>>;
    async fn find_by_slug(&self, slug: &str) -> anyhow::Result<Option<Service>>;
}
