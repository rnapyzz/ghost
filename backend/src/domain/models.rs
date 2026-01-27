use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// -------------------------------
// 1. ノード定義, データ構造
// -------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Initiative,       // 箱: P/Lを作成するために子ノードを合計する
    Project,          // 箱: P/Lを作成するために子ノードを合計する
    SubProject,       // 箱: P/Lを作成するために子ノードを合計する
    Job,              // 実体: Entryを直接持つ
    AdjustmentBuffer, // 実体: Entryを直接持つ
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanNode {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub display_order: i32, // 表示における並び順

    pub scenario_id: Uuid, // 計画のスナップショットID
    pub lineage_id: Uuid,  // 世代を超えて同じ意味のノードであることを認識するキー

    pub node_type: NodeType,     // ツリー構造のノードの種類
    pub parent_id: Option<Uuid>, // ツリー構造の親ノード, Rootの場合はNone

    // どのserviceに紐づくか
    // NodeTypeが実体タイプの場合はSome
    // NodeTypeが箱タイプの場合はNone
    pub service_id: Option<Uuid>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub id: Uuid,
    pub name: String, // e.g. "2026年度 期初計画"
    pub description: Option<String>,

    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_locked: bool, // シナリオの締めフラグ

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

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

// -------------------------------
// 4. service
// -------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub display_order: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
