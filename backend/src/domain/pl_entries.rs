use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
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

#[async_trait::async_trait]
pub trait PlEntryRepository: Send + Sync {
    async fn find_by_cell(
        &self,
        node_id: Uuid,
        account_item_id: Uuid,
        target_month: NaiveDate,
        category: &EntryCategory,
    ) -> anyhow::Result<Option<PlEntry>>;

    async fn create(&self, entry: &PlEntry) -> anyhow::Result<PlEntry>;

    async fn update(&self, entry: &PlEntry) -> anyhow::Result<PlEntry>;

    async fn find_by_node(
        &self,
        node_id: Uuid,
        category: &EntryCategory,
    ) -> anyhow::Result<Vec<PlEntry>>;
}
