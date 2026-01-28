use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// -------------------------------
// 3. P/L entries
// -------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryCategory {
    Plan,
    Result,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
