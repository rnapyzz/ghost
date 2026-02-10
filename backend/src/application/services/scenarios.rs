use crate::domain::pl_entries::{PlEntry, PlEntryRepository};
use crate::domain::plan_nodes::{PlanNode, PlanNodeRepository};
use crate::domain::scenarios::{Scenario, ScenarioRepository};
use chrono::{NaiveDate, Utc};
use std::collections::HashMap;
use uuid::Uuid;

pub struct ScenarioService<S, N, E> {
    scenario_repo: S,
    node_repo: N,
    entry_repo: E,
}

impl<S, N, E> ScenarioService<S, N, E>
where
    S: ScenarioRepository,
    N: PlanNodeRepository,
    E: PlEntryRepository,
{
    pub fn new(scenario_repo: S, node_repo: N, entry_repo: E) -> Self {
        Self {
            scenario_repo,
            node_repo,
            entry_repo,
        }
    }

    pub async fn create(
        &self,
        name: String,
        description: Option<String>,
        start_date: NaiveDate,
        end_date: NaiveDate,
        user_id: Uuid,
    ) -> anyhow::Result<Scenario> {
        let scenario = Scenario::new(name, description, start_date, end_date, user_id)?;

        let created = self.scenario_repo.create(&scenario).await?;

        Ok(created)
    }

    pub async fn list_all(&self) -> anyhow::Result<Vec<Scenario>> {
        self.scenario_repo.find_all().await
    }

    pub async fn activate(&self, id: Uuid) -> anyhow::Result<()> {
        self.scenario_repo.set_current(id).await
    }

    pub async fn rollover(
        &self,
        source_scenario_id: Uuid,
        new_name: String,
        new_start_date: NaiveDate,
        new_end_date: NaiveDate,
        user_id: Uuid,
    ) -> anyhow::Result<Scenario> {
        let source_scenario = self
            .scenario_repo
            .find_by_id(source_scenario_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Source scenario not found"))?;

        let new_scenario = self
            .create(
                new_name,
                Some(format!("Rollover from {}", source_scenario.name)),
                new_start_date,
                new_end_date,
                user_id,
            )
            .await?;

        let old_nodes = self
            .node_repo
            .find_by_scenario_id(source_scenario_id)
            .await?;

        let mut id_map: HashMap<Uuid, Uuid> = HashMap::new();
        let mut new_nodes: Vec<PlanNode> = Vec::new();

        for old_node in &old_nodes {
            let new_id = Uuid::new_v4();
            id_map.insert(old_node.id, new_id);
        }

        for old_node in &old_nodes {
            let new_id = *id_map.get(&old_node.id).unwrap();

            let new_parent_id = old_node.parent_id.and_then(|pid| id_map.get(&pid).copied());
            let new_node = PlanNode {
                id: new_id,
                scenario_id: new_scenario.id,
                parent_id: new_parent_id,
                lineage_id: old_node.lineage_id,
                title: old_node.title.clone(),
                description: old_node.description.clone(),
                node_type: old_node.node_type.clone(),
                display_order: old_node.display_order,
                service_id: old_node.service_id,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by: user_id,
                updated_by: user_id,
                deleted_at: None,
                deleted_by: None,
            };

            new_nodes.push(new_node);
        }

        self.node_repo.create_many(new_nodes).await?;

        let old_node_ids: Vec<Uuid> = old_nodes.iter().map(|n| n.id).collect();
        let old_entries = self.entry_repo.find_by_node_ids(old_node_ids).await?;

        let mut new_entries: Vec<PlEntry> = Vec::new();
        for old_entry in old_entries {
            if let Some(&new_node_id) = id_map.get(&old_entry.node_id) {
                let new_entry = PlEntry {
                    id: Uuid::new_v4(),
                    target_month: old_entry.target_month,
                    entry_category: old_entry.entry_category.clone(),
                    node_id: new_node_id,
                    account_item_id: old_entry.account_item_id,
                    amount: old_entry.amount,
                    description: old_entry.description.clone(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    created_by: user_id,
                    updated_by: user_id,
                };
                new_entries.push(new_entry);
            }
        }

        self.entry_repo.create_many(new_entries).await?;

        self.activate(new_scenario.id).await?;

        Ok(new_scenario)
    }
}
