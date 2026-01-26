use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// -------------------------------
// 1. ノード定義, データ構造
// -------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Initiative,       // 箱
    Project,          // 箱
    SubProject,       // 箱
    Job,              // 実体
    AdjustmentBuffer, // 実体
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanNode {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub display_order: i32, // 表示における並び順

    pub node_type: NodeType,     // ツリー構造のノードの種類
    pub parent_id: Option<Uuid>, // ツリー構造の親ノード, Rootの場合はNone

    // どのserviceに紐づくか
    // NodeTypeが実体タイプの場合はSome
    // NodeTypeが箱タイプの場合はNone
    pub service_id: Option<Uuid>,
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
pub struct PlEntry {
    pub id: Uuid,

    pub target_month: NaiveDate, // 年月
    pub node_id: Uuid,           // どのノードに紐づくP/Lか
    pub account_item_id: Uuid,   // 科目
    pub amount: Decimal,         // 金額

    pub description: Option<String>, // メモ
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

// -------------------------------
// 5. scenario
// -------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Scenario {
    MasterPlan,  // 期初計画
    RevisedPlan, // 改訂期初計画
    ExecPlan,    // 実行計画
    Actual,      // 実績
}
