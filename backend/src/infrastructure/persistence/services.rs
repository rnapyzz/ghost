use sqlx::PgPool;

use crate::domain::services::{Service, ServiceRepository};

#[derive(Debug, Clone)]
pub struct ServiceRepositoryImpl {
    pool: PgPool,
}

impl ServiceRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ServiceRepository for ServiceRepositoryImpl {
    async fn create(&self, service: &Service) -> anyhow::Result<Service> {
        let rec = sqlx::query_as!(
            Service,
            r#"
            INSERT INTO services
            (
                id,
                name,
                slug,
                display_order,
                created_at,
                updated_at,
                deleted_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            service.id,
            service.name,
            service.slug,
            service.display_order,
            service.created_at,
            service.updated_at,
            service.deleted_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Service>> {
        let recs = sqlx::query_as!(
            Service,
            r#"
            SELECT * FROM services
            WHERE deleted_at IS NULL
            ORDER BY display_order
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recs)
    }

    async fn find_by_slug(&self, slug: &str) -> anyhow::Result<Option<Service>> {
        let rec = sqlx::query_as!(
            Service,
            r#"
            SELECT * FROM services
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(rec)
    }
}
