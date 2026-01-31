use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    history::PlEntryHistory,
    pl_entries::{EntryCategory, PlEntry, PlEntryRepository},
};

#[derive(Debug, Clone)]
pub struct PlEntryRepositoryImpl {
    pool: PgPool,
}

impl PlEntryRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PlEntryRepository for PlEntryRepositoryImpl {
    async fn find_by_cell(
        &self,
        node_id: Uuid,
        account_item_id: Uuid,
        target_month: NaiveDate,
        category: &EntryCategory,
    ) -> anyhow::Result<Option<PlEntry>> {
        let rec = sqlx::query_as!(
            PlEntry,
            r#"
            SELECT
                id,
                target_month,
                entry_category as "entry_category: _",
                node_id,
                account_item_id,
                amount,
                description,
                created_at,
                updated_at,
                created_by,
                updated_by
            FROM pl_entries
            WHERE node_id = $1
              AND account_item_id = $2
              AND target_month = $3
              AND entry_category = $4::entry_category
            "#,
            node_id,
            account_item_id,
            target_month,
            category as _
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn create(&self, entry: &PlEntry) -> anyhow::Result<PlEntry> {
        let rec = sqlx::query_as!(
            PlEntry,
            r#"
            INSERT INTO pl_entries
            (
                id,
                target_month,
                entry_category,
                node_id,
                account_item_id,
                amount,
                description,
                created_at,
                updated_at,
                created_by,
                updated_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING
                id,
                target_month,
                entry_category as "entry_category: _",
                node_id,
                account_item_id,
                amount,
                description,
                created_at,
                updated_at,
                created_by,
                updated_by
            "#,
            entry.id,
            entry.target_month,
            entry.entry_category as _,
            entry.node_id,
            entry.account_item_id,
            entry.amount,
            entry.description,
            entry.created_at,
            entry.updated_at,
            entry.created_by,
            entry.updated_by,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn update(&self, entry: &PlEntry) -> anyhow::Result<PlEntry> {
        let rec = sqlx::query_as!(
            PlEntry,
            r#"
            UPDATE pl_entries
            SET
                amount = $1,
                description = $2,
                updated_at = $3,
                updated_by = $4
            WHERE id = $5
            RETURNING
                id,
                target_month,
                entry_category as "entry_category: _",
                node_id,
                account_item_id,
                amount,
                description,
                created_at,
                updated_at,
                created_by,
                updated_by
            "#,
            entry.amount,
            entry.description,
            entry.updated_at,
            entry.updated_by,
            entry.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn find_by_node(
        &self,
        node_id: Uuid,
        category: &EntryCategory,
    ) -> anyhow::Result<Vec<PlEntry>> {
        let recs = sqlx::query_as!(
            PlEntry,
            r#"
            SELECT
                id,
                target_month,
                entry_category as "entry_category: _",
                node_id,
                account_item_id,
                amount,
                description,
                created_at,
                updated_at,
                created_by,
                updated_by
            FROM pl_entries
            WHERE node_id = $1 AND entry_category = $2::entry_category
            ORDER BY target_month ASC, account_item_id ASC
            "#,
            node_id,
            category as _
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recs)
    }
}
