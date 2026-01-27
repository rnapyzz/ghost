use sqlx::PgPool;

use crate::domain::account_items::{AccountItem, AccountItemRepository};

#[derive(Debug, Clone)]
pub struct AccountItemRepositoryImpl {
    pool: PgPool,
}

impl AccountItemRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AccountItemRepository for AccountItemRepositoryImpl {
    async fn create(&self, item: &AccountItem) -> anyhow::Result<AccountItem> {
        let account_item = sqlx::query_as!(
            AccountItem,
            r#"
            INSERT INTO account_items
            (
                id,
                name,
                code,
                description,
                account_type,
                display_order,
                created_at,
                updated_at,
                deleted_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                id,
                name,
                code,
                description,
                account_type as "account_type: _",
                display_order,
                created_at,
                updated_at,
                deleted_at
            "#,
            item.id,
            item.name,
            item.code,
            item.description,
            item.account_type as _,
            item.display_order,
            item.created_at,
            item.updated_at,
            item.deleted_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(account_item)
    }

    async fn find_all(&self) -> anyhow::Result<Vec<AccountItem>> {
        let account_items = sqlx::query_as!(
            AccountItem,
            r#"
            SELECT
                id,
                name,
                code,
                description,
                account_type as "account_type: _",
                display_order,
                created_at,
                updated_at,
                deleted_at
            FROM account_items
            WHERE deleted_at IS NULL
            ORDER BY display_order ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(account_items)
    }
}
