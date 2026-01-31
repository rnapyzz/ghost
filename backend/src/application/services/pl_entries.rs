use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    domain::{
        history::{ChangeType, PlEntryHistory, PlEntryHistoryRepository},
        pl_entries::{EntryCategory, PlEntry, PlEntryRepository},
        plan_nodes::PlanNodeRepository,
    },
    infrastructure::persistence::history::PlEntryHistoryRepositoryImpl,
};

pub struct PlEntryService<R: PlEntryRepository, N: PlanNodeRepository, H: PlEntryHistoryRepository>
{
    entry_repo: R,
    node_repo: N,
    history_repo: H,
}

impl<R: PlEntryRepository, N: PlanNodeRepository, H: PlEntryHistoryRepository>
    PlEntryService<R, N, H>
{
    pub fn new(entry_repo: R, node_repo: N, history_repo: H) -> Self {
        Self {
            entry_repo,
            node_repo,
            history_repo,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn save_entry(
        &self,
        node_id: Uuid,
        account_item_id: Uuid,
        target_month: NaiveDate,
        entry_category: EntryCategory,
        amount: Decimal,
        description: Option<String>,
        user_id: Uuid,
    ) -> anyhow::Result<PlEntry> {
        // ノードの種類チェック
        let node = self
            .node_repo
            .find_by_id(node_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        if !node.node_type.is_entity() {
            return Err(anyhow::anyhow!(
                "Cannot input entries to Container nodes (Initiative/Project/SubProject)"
            ));
        }

        // 既存エントリーの検索
        let existing_entries = self
            .entry_repo
            .find_by_cell(node_id, account_item_id, target_month, &entry_category)
            .await?;

        if let Some(mut entry) = existing_entries {
            // update
            // 履歴保存用帯ジェクトを作成
            let history = PlEntryHistory::new(
                entry.id,
                ChangeType::Update,
                Some(entry.amount),
                amount,
                user_id,
                Some("API request".to_string()),
            );

            // update処理
            entry.amount = amount;
            entry.description = description;
            entry.updated_at = Utc::now();
            entry.updated_by = user_id;

            let updated = self.entry_repo.update(&entry).await?;

            // 履歴を保存
            self.history_repo.create(&history).await?;

            Ok(updated)
        } else {
            // create
            let new_entry = PlEntry::new(
                target_month,
                entry_category,
                node_id,
                account_item_id,
                amount,
                description,
                user_id,
            );
            let created = self.entry_repo.create(&new_entry).await?;

            // 履歴保存用オブジェクトを作成
            let history = PlEntryHistory::new(
                created.id,
                ChangeType::Create,
                None,
                created.amount,
                user_id,
                Some("API request".to_string()),
            );

            self.history_repo.create(&history).await?;

            Ok(created)
        }
    }

    pub async fn list_by_node(
        &self,
        node_id: Uuid,
        category: EntryCategory,
    ) -> anyhow::Result<Vec<PlEntry>> {
        self.entry_repo.find_by_node(node_id, &category).await
    }
}
