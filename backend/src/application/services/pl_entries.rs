use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::domain::scenarios::ScenarioRepository;
use crate::{
    domain::{
        history::{ChangeType, PlEntryHistory, PlEntryHistoryRepository},
        pl_entries::{EntryCategory, PlEntry, PlEntryRepository},
        plan_nodes::PlanNodeRepository,
    },
    presentation::dtos::SavePlEntryRequest,
};

pub struct PlEntryService<
    R: PlEntryRepository,
    N: PlanNodeRepository,
    H: PlEntryHistoryRepository,
    S: ScenarioRepository,
> {
    pool: PgPool,
    entry_repo: R,
    node_repo: N,
    history_repo: H,
    scenario_repo: S,
}

impl<
    R: PlEntryRepository,
    N: PlanNodeRepository,
    H: PlEntryHistoryRepository,
    S: ScenarioRepository,
> PlEntryService<R, N, H, S>
{
    pub fn new(
        pool: PgPool,
        entry_repo: R,
        node_repo: N,
        history_repo: H,
        scenario_repo: S,
    ) -> Self {
        Self {
            pool,
            entry_repo,
            node_repo,
            history_repo,
            scenario_repo,
        }
    }

    // 書き込み権限とノードタイプのチェックを行うヘルパーメソッド
    async fn ensure_writable(&self, node_id: Uuid) -> anyhow::Result<()> {
        // 存在確認
        let node = self
            .node_repo
            .find_by_id(node_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Node not found"))?;

        // ノードのタイプチェック
        if !node.node_type.is_entity() {
            return Err(anyhow::anyhow!(
                "Cannot input entries to Container nodes (Initiative/Project/SubProject)"
            ));
        }

        // シナリオの書き込み権限チェック
        let scenario = self
            .scenario_repo
            .find_by_id(node.scenario_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Scenario not found"))?;

        if !scenario.is_current {
            return Err(anyhow::anyhow!(
                "Read-Only: Past scenarios cannot be edited"
            ));
        }

        Ok(())
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
        // チェックを実施
        self.ensure_writable(node_id).await?;

        // トランザクション開始
        let mut tx = self.pool.begin().await?;

        // ロジックの実行
        let result = self
            .save_entry_logic(
                &mut tx,
                node_id,
                account_item_id,
                target_month,
                entry_category,
                amount,
                description,
                user_id,
            )
            .await?;

        // コミット
        tx.commit().await?;

        Ok(result)
    }

    pub async fn save_bulk(
        &self,
        requests: Vec<SavePlEntryRequest>,
        user_id: Uuid,
    ) -> anyhow::Result<()> {
        // トランザクション開始
        let mut tx = self.pool.begin().await?;

        for req in requests {
            // ノードの種類チェック
            self.ensure_writable(req.node_id).await?;

            // ロジックの実行
            self.save_entry_logic(
                &mut tx,
                req.node_id,
                req.account_item_id,
                req.target_month,
                req.entry_category,
                req.amount,
                req.description,
                user_id,
            )
            .await?;
        }

        // コミット
        tx.commit().await?;
        Ok(())
    }

    pub async fn save_entry_logic(
        &self,
        tx: &mut PgConnection,
        node_id: Uuid,
        account_item_id: Uuid,
        target_month: NaiveDate,
        entry_category: EntryCategory,
        amount: Decimal,
        description: Option<String>,
        user_id: Uuid,
    ) -> anyhow::Result<PlEntry> {
        let existing_entries = self
            .entry_repo
            .find_by_cell(tx, node_id, account_item_id, target_month, &entry_category)
            .await?;

        if let Some(mut entry) = existing_entries {
            // update

            // 変更がない場合はスキップ
            if entry.amount == amount && entry.description == description {
                return Ok(entry);
            }
            // 履歴保存用帯ジェクトを作成
            let history = PlEntryHistory::new(
                entry.id,
                ChangeType::Update,
                Some(entry.amount),
                amount,
                user_id,
                Some("Bulk/API".to_string()),
            );

            // update処理
            entry.amount = amount;
            entry.description = description;
            entry.updated_at = Utc::now();
            entry.updated_by = user_id;

            let updated = self.entry_repo.update(tx, &entry).await?;

            // 履歴を保存
            self.history_repo.create(tx, &history).await?;

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
            let created = self.entry_repo.create(tx, &new_entry).await?;

            // 履歴保存用オブジェクトを作成
            let history = PlEntryHistory::new(
                created.id,
                ChangeType::Create,
                None,
                created.amount,
                user_id,
                Some("Bulk/API".to_string()),
            );

            self.history_repo.create(tx, &history).await?;

            Ok(created)
        }
    }

    pub async fn list_by_node(
        &self,
        node_id: Uuid,
        category: EntryCategory,
    ) -> anyhow::Result<Vec<PlEntry>> {
        let mut conn = self.pool.acquire().await?;
        self.entry_repo
            .find_by_node(&mut conn, node_id, &category)
            .await
    }

    pub async fn list_by_scenario(&self, scenario_id: Uuid) -> anyhow::Result<Vec<PlEntry>> {
        self.entry_repo.find_by_scenario_id(scenario_id).await
    }
}
