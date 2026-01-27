use crate::domain::account_items::{AccountItem, AccountItemRepository, AccountType};

pub struct AccountItemService<R: AccountItemRepository> {
    repository: R,
}

impl<R: AccountItemRepository> AccountItemService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        name: String,
        code: String,
        description: Option<String>,
        account_type: AccountType,
        display_order: i32,
    ) -> anyhow::Result<AccountItem> {
        let item = AccountItem::new(name, code, description, account_type, display_order)?;

        let created_item = self.repository.create(&item).await?;

        Ok(created_item)
    }

    pub async fn list_all(&self) -> anyhow::Result<Vec<AccountItem>> {
        let items = self.repository.find_all().await?;
        Ok(items)
    }
}
