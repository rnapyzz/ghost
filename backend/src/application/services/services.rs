use crate::domain::services::{Service, ServiceRepository};

pub struct ServiceService<R: ServiceRepository> {
    repository: R,
}

impl<R: ServiceRepository> ServiceService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        name: String,
        slug: String,
        display_order: i32,
    ) -> anyhow::Result<Service> {
        // slugの重複チェック
        if self.repository.find_by_slug(&slug).await?.is_some() {
            return Err(anyhow::anyhow!("Slug already exists"));
        }

        let service = Service::new(name, slug, display_order)?;

        let created = self.repository.create(&service).await?;

        Ok(created)
    }

    pub async fn list_all(&self) -> anyhow::Result<Vec<Service>> {
        self.repository.find_all().await
    }
}
