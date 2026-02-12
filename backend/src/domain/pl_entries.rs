use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "entry_category")]
pub enum EntryCategory {
    Plan,
    Result,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PlEntry {
    pub id: Uuid,
    pub target_month: NaiveDate,       // 年月
    pub entry_category: EntryCategory, // 「確定値」か「計画値」か
    pub node_id: Uuid,                 // どのノードに紐づくP/Lか
    pub account_item_id: Uuid,         // 科目
    pub amount: Decimal,               // 金額
    pub description: Option<String>,   // メモ

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

impl PlEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        target_month: NaiveDate,
        entry_category: EntryCategory,
        node_id: Uuid,
        account_item_id: Uuid,
        amount: Decimal,
        description: Option<String>,
        user_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            target_month,
            entry_category,
            node_id,
            account_item_id,
            amount,
            description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: user_id,
            updated_by: user_id,
        }
    }
}

#[async_trait::async_trait]
pub trait PlEntryRepository: Send + Sync {
    async fn find_by_cell(
        &self,
        tx: &mut PgConnection,
        node_id: Uuid,
        account_item_id: Uuid,
        target_month: NaiveDate,
        category: &EntryCategory,
    ) -> anyhow::Result<Option<PlEntry>>;

    async fn create(&self, tx: &mut PgConnection, entry: &PlEntry) -> anyhow::Result<PlEntry>;

    async fn update(&self, tx: &mut PgConnection, entry: &PlEntry) -> anyhow::Result<PlEntry>;

    async fn find_by_node(
        &self,
        tx: &mut PgConnection,
        node_id: Uuid,
        category: &EntryCategory,
    ) -> anyhow::Result<Vec<PlEntry>>;

    async fn find_by_node_ids(&self, node_ids: Vec<Uuid>) -> anyhow::Result<Vec<PlEntry>>;
    async fn find_by_scenario_id(&self, scenario_id: Uuid) -> anyhow::Result<Vec<PlEntry>>;
    async fn create_many(&self, entries: Vec<PlEntry>) -> anyhow::Result<()>;
}
