// -------------------------------
// Audit Log
// -------------------------------

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
