use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::domain::plan_nodes::{PlanNode, PlanNodeRepository, UpdatePlanNodeParams};

#[derive(Debug, Clone)]
pub struct PlanNodeRepositoryImpl {
    pool: PgPool,
}

impl PlanNodeRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PlanNodeRepository for PlanNodeRepositoryImpl {
    async fn create(&self, node: &PlanNode) -> anyhow::Result<PlanNode> {
        let rec = sqlx::query_as!(
            PlanNode,
            r#"
            INSERT INTO plan_nodes
            (
                id,
                scenario_id,
                parent_id,
                lineage_id,
                title,
                description,
                node_type,
                display_order,
                service_id,
                created_at,
                updated_at,
                created_by,
                updated_by,
                deleted_at,
                deleted_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING
                id,
                scenario_id,
                parent_id,
                lineage_id,
                title,
                description,
                node_type as "node_type: _",
                display_order,
                service_id,
                created_at,
                updated_at,
                created_by,
                updated_by,
                deleted_at,
                deleted_by
            "#,
            node.id,
            node.scenario_id,
            node.parent_id,
            node.lineage_id,
            node.title,
            node.description,
            node.node_type as _,
            node.display_order,
            node.service_id,
            node.created_at,
            node.updated_at,
            node.created_by,
            node.updated_by,
            node.deleted_at,
            node.deleted_by
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn find_recent(&self, limit: i64) -> anyhow::Result<Vec<PlanNode>> {
        let recs = sqlx::query_as!(
            PlanNode,
            r#"
            SELECT
                id,
                scenario_id,
                parent_id,
                lineage_id,
                title,
                description,
                node_type as "node_type: _",
                display_order,
                service_id,
                created_at,
                updated_at,
                created_by,
                updated_by,
                deleted_at,
                deleted_by
            FROM plan_nodes
            ORDER BY created_at DESC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recs)
    }

    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<PlanNode>> {
        let rec = sqlx::query_as!(
            PlanNode,
            r#"
            SELECT
                id,
                scenario_id,
                parent_id,
                lineage_id,
                title,
                description,
                node_type as "node_type: _",
                display_order,
                service_id,
                created_at,
                updated_at,
                created_by,
                updated_by,
                deleted_at,
                deleted_by
            FROM plan_nodes
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn find_by_scenario_id(&self, scenario_id: Uuid) -> anyhow::Result<Vec<PlanNode>> {
        let recs = sqlx::query_as!(
            PlanNode,
            r#"
            SELECT
                id,
                scenario_id,
                parent_id,
                lineage_id,
                title,
                description,
                node_type as "node_type: _",
                display_order,
                service_id,
                created_at,
                updated_at,
                created_by,
                updated_by,
                deleted_at,
                deleted_by
            FROM plan_nodes
            WHERE scenario_id = $1 AND deleted_at IS NULL
            ORDER BY display_order ASC, created_at ASC
            "#,
            scenario_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recs)
    }

    async fn update(
        &self,
        id: Uuid,
        params: UpdatePlanNodeParams,
        updated_by: Uuid,
    ) -> anyhow::Result<PlanNode> {
        let mut builder =
            QueryBuilder::new("UPDATE plan_nodes SET updated_at = NOW(), updated_by = ");
        builder.push_bind(updated_by);

        if let Some(title) = params.title {
            builder.push(", title = ");
            builder.push_bind(title);
        }

        if let Some(description) = params.description {
            builder.push(", description = ");
            builder.push_bind(description);
        }

        if let Some(display_order) = params.display_order {
            builder.push(", display_order = ");
            builder.push_bind(display_order);
        }

        builder.push(" WHERE id = ");
        builder.push_bind(id);
        builder.push(" RETURNING *");

        let node = builder
            .build_query_as::<PlanNode>()
            .fetch_one(&self.pool)
            .await?;

        Ok(node)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        // 子ノードが存在するかどうかチェック
        let child_count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM plan_nodes WHERE parent_id = $1")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        if child_count > 0 {
            return Err(anyhow::anyhow!("子ノードが存在するため削除できません。"));
        }

        // 紐づくPlEntryが存在するかどうかチェック
        let entry_count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM pl_entries WHERE plan_node_id = $1")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        if entry_count > 0 {
            return Err(anyhow::anyhow!(
                "数値データが紐づいているため削除できません。"
            ));
        }

        let result = sqlx::query("DELETE FROM plan_nodes WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Plan Node not found."));
        }

        Ok(())
    }
}
