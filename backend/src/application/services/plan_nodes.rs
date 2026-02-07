use uuid::Uuid;

use crate::domain::plan_nodes::{NodeType, PlanNode, PlanNodeRepository, UpdatePlanNodeParams};
use crate::domain::scenarios::ScenarioRepository;
use crate::presentation::dtos::UpdatePlanNodeRequest;

pub struct PlanNodeService<P, S> {
    plan_node_repo: P,
    scenario_repo: S,
}

impl<P, S> PlanNodeService<P, S>
where
    P: PlanNodeRepository,
    S: ScenarioRepository,
{
    pub fn new(plan_node_repo: P, scenario_repo: S) -> Self {
        Self {
            plan_node_repo,
            scenario_repo,
        }
    }

    async fn ensure_scenario_is_writable(&self, scenario_id: Uuid) -> anyhow::Result<()> {
        let scenario = self
            .scenario_repo
            .find_by_id(scenario_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Scenario not found"))?;

        if !scenario.is_current {
            return Err(anyhow::anyhow!(
                "作成中のシナリオ以外は編集できません（Read-Only）"
            ));
        }

        Ok(())
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
        self.ensure_scenario_is_writable(scenario_id).await?;

        // 親Nodeがある場合のバリデーション
        if let Some(pid) = parent_id {
            let parent = self
                .plan_node_repo
                .find_by_id(pid)
                .await?
                .ok_or_else(|| anyhow::anyhow!("Parent node not found"))?;

            // シナリオの一致確認
            if parent.scenario_id != scenario_id {
                return Err(anyhow::anyhow!(
                    "Parent node belongs to a different scenario"
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

        let created = self.plan_node_repo.create(&new_node).await?;

        Ok(created)
    }

    pub async fn list_recent(&self, limit: i64) -> anyhow::Result<Vec<PlanNode>> {
        self.plan_node_repo.find_recent(limit).await
    }

    pub async fn list_by_scenario(&self, scenario_id: Uuid) -> anyhow::Result<Vec<PlanNode>> {
        self.plan_node_repo.find_by_scenario_id(scenario_id).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdatePlanNodeRequest,
        updated_by: Uuid,
    ) -> anyhow::Result<PlanNode> {
        let current_node = self
            .plan_node_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("PlanNode not found"))?;

        self.ensure_scenario_is_writable(current_node.scenario_id)
            .await?;

        // DTO から Domain Paramsへ変換
        let params: UpdatePlanNodeParams = req.into();

        self.plan_node_repo.update(id, params, updated_by).await
    }

    pub async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        let current_node = self
            .plan_node_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("PlanNode not found"))?;

        self.ensure_scenario_is_writable(current_node.scenario_id)
            .await?;

        self.plan_node_repo.delete(id).await
    }
}
