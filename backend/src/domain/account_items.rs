use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_type")]
pub enum AccountType {
    Revenue,
    CostOfGoodsSold,
    SellingGeneralAdmin,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AccountItem {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub account_type: AccountType,
    pub display_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl AccountItem {
    pub fn new(
        name: String,
        code: String,
        description: Option<String>,
        account_type: AccountType,
        display_order: i32,
    ) -> anyhow::Result<Self> {
        if name.trim().is_empty() {
            return Err(anyhow::anyhow!("Name cannot be empty"));
        }

        if code.trim().is_empty() {
            return Err(anyhow::anyhow!("Code cannot be empty"));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            code,
            description,
            account_type,
            display_order,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }
}

#[async_trait::async_trait]
pub trait AccountItemRepository: Send + Sync {
    async fn create(&self, item: &AccountItem) -> anyhow::Result<AccountItem>;
    async fn find_all(&self) -> anyhow::Result<Vec<AccountItem>>;
}
