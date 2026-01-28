use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "plan_node_type")]
pub enum NodeType {
    Initiative,       // 箱: P/Lを作成するために子ノードを合計する
    Project,          // 箱: P/Lを作成するために子ノードを合計する
    SubProject,       // 箱: P/Lを作成するために子ノードを合計する
    Job,              // 実体: Entryを直接持つ
    AdjustmentBuffer, // 実体: Entryを直接持つ
}

impl NodeType {
    /// Nodeの親子関係ルール
    pub fn can_be_child_of(&self, parent_type: &NodeType) -> bool {
        match (parent_type, self) {
            (NodeType::Initiative, NodeType::Project) => true,

            (NodeType::Project, NodeType::SubProject) => true,
            (NodeType::Project, NodeType::Job) => true,
            (NodeType::Project, NodeType::AdjustmentBuffer) => true,

            (NodeType::SubProject, NodeType::SubProject) => true,
            (NodeType::SubProject, NodeType::Job) => true,
            (NodeType::SubProject, NodeType::AdjustmentBuffer) => true,

            _ => false,
        }
    }

    /// Rootになれるかどうかの判定
    pub fn can_be_root(&self) -> bool {
        matches!(self, NodeType::Initiative)
    }

    /// 実体系のタイプかどうか
    /// 実体系のタイプはEntryを持ち、service_idを持たなくてはいけないが、子Nodeを持つことはできない
    pub fn is_entity(&self) -> bool {
        matches!(self, NodeType::Job | NodeType::AdjustmentBuffer)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PlanNode {
    pub id: Uuid,
    pub scenario_id: Uuid,       // 計画のスナップショットID
    pub parent_id: Option<Uuid>, // ツリー構造の親ノード, Rootの場合はNone
    pub lineage_id: Uuid,        // 世代を超えて同じ意味のノードであることを認識するキー

    pub title: String,
    pub description: Option<String>,
    pub node_type: NodeType, // ツリー構造のノードの種類
    pub display_order: i32,  // 表示における並び順

    // どのserviceに紐づくか
    // NodeTypeが実体タイプの場合はSome
    // NodeTypeが箱タイプの場合はNone
    pub service_id: Option<Uuid>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,

    #[serde(skip)]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(skip)]
    pub deleted_by: Option<Uuid>,
}

impl PlanNode {
    pub fn new(
        scenario_id: Uuid,
        parent_id: Option<Uuid>,
        lineage_id: Option<Uuid>,
        title: String,
        description: Option<String>,
        node_type: NodeType,
        display_order: i32,
        service_id: Option<Uuid>,
        user_id: Uuid,
    ) -> anyhow::Result<Self> {
        if title.trim().is_empty() {
            return Err(anyhow::anyhow!("Title cannot be empty"));
        }

        // rootノードのチェック
        if parent_id.is_none() && !node_type.can_be_root() {
            return Err(anyhow::anyhow!("Only 'Initiative' can bea root node"));
        }

        // service_idの整合チェック
        if node_type.is_entity() {
            if service_id.is_none() {
                return Err(anyhow::anyhow!(
                    "Service ID is required for Job or AdjustmentBuffer"
                ));
            }
        } else {
            if service_id.is_some() {
                return Err(anyhow::anyhow!(
                    "Service ID must be None for Container nodes (Initiative, Project, etc.)"
                ));
            }
        }

        Ok(Self {
            id: Uuid::new_v4(),
            scenario_id,
            parent_id,
            lineage_id: lineage_id.unwrap_or_else(Uuid::new_v4),
            title,
            description,
            node_type,
            display_order,
            service_id,
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
pub trait PlanNodeRepository: Send + Sync {
    async fn create(&self, node: &PlanNode) -> anyhow::Result<PlanNode>;
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<PlanNode>>;
    async fn find_by_scenario_id(&self, scenario_id: Uuid) -> anyhow::Result<Vec<PlanNode>>;
}
