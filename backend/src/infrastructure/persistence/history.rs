use sqlx::PgPool;

use crate::domain::history::{PlEntryHistory, PlEntryHistoryRepository};

#[derive(Debug, Clone)]
pub struct PlEntryHistoryRepositoryImpl {
    pool: PgPool,
}

impl PlEntryHistoryRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PlEntryHistoryRepository for PlEntryHistoryRepositoryImpl {
    async fn create(&self, history: &PlEntryHistory) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO pl_entry_histories
            (
                id,
                entry_id,
                change_type,
                previous_amount,
                new_amount,
                changed_at,
                changed_by,
                operation_source
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            history.id,
            history.entry_id,
            history.change_type as _,
            history.previous_amount,
            history.new_amount,
            history.changed_at,
            history.changed_by,
            history.operation_source,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
