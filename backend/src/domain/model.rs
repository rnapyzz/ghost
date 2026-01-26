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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,

    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_locked: bool,
    pub created_at: DateTime<Utc>,
}

// -------------------------------
// 2. 勘定科目定義, メタデータ
// -------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Revenue,
    CostOfGoodsSold,
    SellingGeneralAdmin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountItem {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub account_type: AccountType,
    pub display_order: i32,
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
}
