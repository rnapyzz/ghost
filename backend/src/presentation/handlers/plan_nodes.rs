use axum::extract::Path;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use validator::Validate;

use crate::infrastructure::persistence::scenarios::ScenarioRepositoryImpl;
use crate::presentation::dtos::UpdatePlanNodeRequest;
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

    let plan_node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());
    let service = PlanNodeService::new(plan_node_repo, scenario_repo);

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
    let plan_node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());
    let service = PlanNodeService::new(plan_node_repo, scenario_repo);

    let result = match query.scenario_id {
        Some(id) => service.list_by_scenario(id).await,
        None => service.list_recent(100).await,
    };

    match result {
        Ok(nodes) => Ok((StatusCode::OK, Json(nodes))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePlanNodeRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let plan_node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());
    let service = PlanNodeService::new(plan_node_repo, scenario_repo);

    match service.update(id, req, auth_user.id).await {
        Ok(node) => Ok(Json(node)),
        Err(e) => {
            let err_msg = e.to_string();
            if err_msg.contains("not found") {
                Err((StatusCode::NOT_FOUND, err_msg))
            } else if err_msg.contains("Read-Only") {
                Err((StatusCode::FORBIDDEN, err_msg))
            } else {
                tracing::error!("Plan Node Update Error: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, err_msg))
            }
        }
    }
}

pub async fn delete(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let plan_node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());
    let service = PlanNodeService::new(plan_node_repo, scenario_repo);

    match service.delete(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            let err_msg = e.to_string();
            if err_msg.contains("not found") {
                Err((StatusCode::NOT_FOUND, err_msg))
            } else if err_msg.contains("削除できません") {
                Err((StatusCode::BAD_REQUEST, err_msg))
            } else if err_msg.contains("Read-Only") {
                Err((StatusCode::FORBIDDEN, err_msg))
            } else {
                tracing::error!("Plan Node Delete Error: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, err_msg))
            }
        }
    }
}
