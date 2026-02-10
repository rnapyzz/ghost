use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use uuid::Uuid;
use validator::Validate;

use crate::infrastructure::persistence::pl_entries::PlEntryRepositoryImpl;
use crate::infrastructure::persistence::plan_nodes::PlanNodeRepositoryImpl;
use crate::presentation::dtos::RolloverScenarioRequest;
use crate::{
    application::services::scenarios::ScenarioService,
    domain::user::UserRole,
    infrastructure::persistence::scenarios::ScenarioRepositoryImpl,
    presentation::{dtos::CreateScenarioRequest, extractors::AuthUser},
    state::AppState,
};

pub async fn create(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateScenarioRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if auth_user.role != UserRole::Admin {
        return Err((StatusCode::FORBIDDEN, "Permission denied".to_string()));
    }

    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());
    let node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let entry_repo = PlEntryRepositoryImpl::new(state.pool.clone());

    let service = ScenarioService::new(scenario_repo, node_repo, entry_repo);

    match service
        .create(
            payload.name,
            payload.description,
            payload.start_date,
            payload.end_date,
            auth_user.id,
        )
        .await
    {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(e) => {
            tracing::error!("Create scenario error: {}", e);
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
    }
}

pub async fn list(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());
    let node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let entry_repo = PlEntryRepositoryImpl::new(state.pool.clone());

    let service = ScenarioService::new(scenario_repo, node_repo, entry_repo);

    match service.list_all().await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn activate(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());
    let node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let entry_repo = PlEntryRepositoryImpl::new(state.pool.clone());

    let service = ScenarioService::new(scenario_repo, node_repo, entry_repo);

    match service.activate(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Error activating scenario: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn rollover(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(source_id): Path<Uuid>,
    Json(payload): Json<RolloverScenarioRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());
    let node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let entry_repo = PlEntryRepositoryImpl::new(state.pool.clone());

    let service = ScenarioService::new(scenario_repo, node_repo, entry_repo);

    match service
        .rollover(
            source_id,
            payload.name,
            payload.start_date,
            payload.end_date,
            auth_user.id,
        )
        .await
    {
        Ok(new_scenario) => Ok((StatusCode::CREATED, Json(new_scenario))),
        Err(e) => {
            tracing::error!("Rollover error: {:?}", e);
            let msg = e.to_string();
            if msg.contains("not found") {
                Err((StatusCode::NOT_FOUND, msg))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, msg))
            }
        }
    }
}
