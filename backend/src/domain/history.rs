use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "change_type")]
pub enum ChangeType {
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PlEntryHistory {
    pub id: Uuid,
    pub entry_id: Uuid,
    pub change_type: ChangeType,

    // 変更前の値
    // Create時にはNone, Update時には値を入れる
    pub previous_amount: Option<Decimal>,

    pub new_amount: Decimal,

    pub changed_at: DateTime<Utc>,
    pub changed_by: Uuid,

    // どのAPIから変更されたか
    pub operation_source: Option<String>,
}

impl PlEntryHistory {
    pub fn new(
        entry_id: Uuid,
        change_type: ChangeType,
        previous_amount: Option<Decimal>,
        new_amount: Decimal,
        user_id: Uuid,
        operation_source: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            entry_id,
            change_type,
            previous_amount,
            new_amount,
            changed_at: Utc::now(),
            changed_by: user_id,
            operation_source,
        }
    }
}

#[async_trait::async_trait]
pub trait PlEntryHistoryRepository: Send + Sync {
    async fn create(&self, hisotry: &PlEntryHistory) -> anyhow::Result<()>;
}
