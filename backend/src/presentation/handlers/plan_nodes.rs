use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

use crate::{
    application::services::plan_nodes::PlanNodeService,
    infrastructure::persistence::plan_nodes::PlanNodeRepositoryImpl,
    presentation::{
        dtos::{CreatePlanNodeRequest, ListPlanNodesQuery},
        extractors::AuthUser,
    },
    state::AppState,
};

pub async fn create(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreatePlanNodeRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let repo = PlanNodeRepositoryImpl::new(state.pool);
    let service = PlanNodeService::new(repo);

    match service
        .create(
            payload.scenario_id,
            payload.parent_id,
            payload.title,
            payload.description,
            payload.node_type,
            payload.display_order,
            payload.service_id,
            auth_user.id,
        )
        .await
    {
        Ok(node) => Ok((StatusCode::CREATED, Json(node))),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("cannot be a child of")
                || msg.contains("Parent node not found")
                || msg.contains("Service ID")
                || msg.contains("Only 'Initiative'")
            {
                tracing::warn!("PlanNode validation failed: {}", msg);
                Err((StatusCode::BAD_REQUEST, msg))
            } else {
                tracing::error!("Failed to create plan node: {}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, msg))
            }
        }
    }
}

pub async fn list(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Query(query): Query<ListPlanNodesQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let repo = PlanNodeRepositoryImpl::new(state.pool);
    let service = PlanNodeService::new(repo);

    let result = match query.scenario_id {
        Some(id) => service.list_by_scenario(id).await,
        None => service.list_recent(100).await,
    };

    match result {
        Ok(nodes) => Ok((StatusCode::OK, Json(nodes))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
