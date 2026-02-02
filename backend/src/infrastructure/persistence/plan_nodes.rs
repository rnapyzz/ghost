use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::plan_nodes::{PlanNode, PlanNodeRepository};

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
}
