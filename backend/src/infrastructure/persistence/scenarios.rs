use crate::domain::scenarios::{Scenario, ScenarioRepository};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ScenarioRepositoryImpl {
    pool: PgPool,
}

impl ScenarioRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ScenarioRepository for ScenarioRepositoryImpl {
    async fn create(&self, scenario: &Scenario) -> anyhow::Result<Scenario> {
        let rec = sqlx::query_as!(
            Scenario,
            r#"
            INSERT INTO scenarios
            (
                id,
                name,
                description,
                start_date,
                end_date,
                is_locked,
                is_current,
                created_at,
                updated_at,
                created_by,
                updated_by,
                deleted_at,
                deleted_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#,
            scenario.id,
            scenario.name,
            scenario.description,
            scenario.start_date,
            scenario.end_date,
            scenario.is_locked,
            scenario.is_current,
            scenario.created_at,
            scenario.updated_at,
            scenario.created_by,
            scenario.updated_by,
            scenario.deleted_at,
            scenario.deleted_by
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Scenario>> {
        let recs = sqlx::query_as!(
            Scenario,
            r#"
            SELECT * FROM scenarios
            WHERE deleted_at IS NULL
            ORDER BY start_date DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recs)
    }

    async fn set_current(&self, id: Uuid) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("UPDATE scenarios SET is_current = FALSE")
            .execute(&mut *tx)
            .await?;

        let result = sqlx::query("UPDATE scenarios SET is_current = TRUE WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        if result.rows_affected() == 0 {
            tx.rollback().await?;
            return Err(anyhow::anyhow!("Scenario not found"));
        }

        tx.commit().await?;
        Ok(())
    }
}
