use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Scenario {
    pub id: Uuid,
    pub name: String, // e.g. "2026年度 期初計画"
    pub description: Option<String>,

    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_locked: bool,  // シナリオの締めフラグ
    pub is_current: bool, // 現在作成中のシナリオかどうかのフラグ

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,

    #[serde(skip)]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(skip)]
    pub deleted_by: Option<Uuid>,
}

impl Scenario {
    pub fn new(
        name: String,
        description: Option<String>,
        start_date: NaiveDate,
        end_date: NaiveDate,
        user_id: Uuid,
    ) -> anyhow::Result<Self> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Name cannot be empty"));
        }

        if start_date > end_date {
            return Err(anyhow::anyhow!("Start date must be before end date"));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description,
            start_date,
            end_date,
            is_locked: false,
            is_current: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: user_id,
            updated_by: user_id,
            deleted_at: None,
            deleted_by: None,
        })
    }
}

#[async_trait::async_trait]
pub trait ScenarioRepository {
    async fn create(&self, scenario: &Scenario) -> anyhow::Result<Scenario>;
    async fn find_all(&self) -> anyhow::Result<Vec<Scenario>>;
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Scenario>>;
    async fn set_current(&self, id: Uuid) -> anyhow::Result<()>;
}
