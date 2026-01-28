use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::scenarios::{Scenario, ScenarioRepository};

pub struct ScenarioService<R: ScenarioRepository> {
    repository: R,
}

impl<R: ScenarioRepository> ScenarioService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
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

        let created = self.repository.create(&scenario).await?;

        Ok(created)
    }

    pub async fn list_all(&self) -> anyhow::Result<Vec<Scenario>> {
        self.repository.find_all().await
    }
}
