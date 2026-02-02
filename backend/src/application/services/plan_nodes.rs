use uuid::Uuid;

use crate::domain::plan_nodes::{NodeType, PlanNode, PlanNodeRepository};

pub struct PlanNodeService<R: PlanNodeRepository> {
    repository: R,
}

impl<R: PlanNodeRepository> PlanNodeService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    #[allow(clippy::too_man_arguments)]
    pub async fn create(
        &self,
        scenario_id: Uuid,
        parent_id: Option<Uuid>,
        title: String,
        description: Option<String>,
        node_type: NodeType,
        display_order: i32,
        service_id: Option<Uuid>,
        user_id: Uuid,
    ) -> anyhow::Result<PlanNode> {
        // 親Nodeがある場合のバリデーション
        if let Some(pid) = parent_id {
            let parent = self
                .repository
                .find_by_id(pid)
                .await?
                .ok_or_else(|| anyhow::anyhow!("Parent node not found"))?;

            // シナリオの一致確認
            if parent.scenario_id != scenario_id {
                return Err(anyhow::anyhow!(
                    "Parent node belongs to a diffrent scenario"
                ));
            }

            // 階層ルールのチェック
            if !node_type.can_be_child_of(&parent.node_type) {
                return Err(anyhow::anyhow!(
                    "Node type '{:?}' cannot be a child of '{:?}'",
                    node_type,
                    parent.node_type
                ));
            }
        }

        // ドメインモデルの生成
        let new_node = PlanNode::new(
            scenario_id,
            parent_id,
            None,
            title,
            description,
            node_type,
            display_order,
            service_id,
            user_id,
        )?;

        let created = self.repository.create(&new_node).await?;

        Ok(created)
    }

    pub async fn list_recent(&self, limit: i64) -> anyhow::Result<Vec<PlanNode>> {
        self.repository.find_recent(limit).await
    }

    pub async fn list_by_scenario(&self, scenario_id: Uuid) -> anyhow::Result<Vec<PlanNode>> {
        self.repository.find_by_scenario_id(scenario_id).await
    }
}
