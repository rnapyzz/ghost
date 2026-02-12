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
use crate::{
    application::services::pl_entries::PlEntryService,
    infrastructure::persistence::{
        history::PlEntryHistoryRepositoryImpl, pl_entries::PlEntryRepositoryImpl,
        plan_nodes::PlanNodeRepositoryImpl,
    },
    presentation::{
        dtos::{BulkSavePlEntryRequest, ListPlEntryQuery, SavePlEntryRequest},
        extractors::AuthUser,
    },
    state::AppState,
};

pub async fn save(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<SavePlEntryRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let entry_repo = PlEntryRepositoryImpl::new(state.pool.clone());
    let node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let history_repo = PlEntryHistoryRepositoryImpl::new(state.pool.clone());
    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());

    let service = PlEntryService::new(
        state.pool.clone(),
        entry_repo,
        node_repo,
        history_repo,
        scenario_repo,
    );

    match service
        .save_entry(
            payload.node_id,
            payload.account_item_id,
            payload.target_month,
            payload.entry_category,
            payload.amount,
            payload.description,
            auth_user.id,
        )
        .await
    {
        Ok(entry) => Ok((StatusCode::OK, Json(entry))),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("Cannot input entries") || msg.contains("Node not found") {
                Err((StatusCode::BAD_REQUEST, msg))
            } else {
                tracing::error!("Save entry error: {}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, msg))
            }
        }
    }
}

pub async fn bulk_save(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<BulkSavePlEntryRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let entry_repo = PlEntryRepositoryImpl::new(state.pool.clone());
    let node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let history_repo = PlEntryHistoryRepositoryImpl::new(state.pool.clone());
    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());

    let service = PlEntryService::new(
        state.pool.clone(),
        entry_repo,
        node_repo,
        history_repo,
        scenario_repo,
    );

    match service.save_bulk(payload.entries, auth_user.id).await {
        Ok(_) => Ok((StatusCode::OK, "Bulk save successful")),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("Read-Only") {
                Err((StatusCode::FORBIDDEN, msg))
            } else if msg.contains("Cannot input entries") {
                Err((StatusCode::BAD_REQUEST, msg))
            } else {
                tracing::error!("Bulk save error: {}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }
    }
}

pub async fn list(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Query(query): Query<ListPlEntryQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entry_repo = PlEntryRepositoryImpl::new(state.pool.clone());
    let node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let history_repo = PlEntryHistoryRepositoryImpl::new(state.pool.clone());
    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());

    let service = PlEntryService::new(
        state.pool.clone(),
        entry_repo,
        node_repo,
        history_repo,
        scenario_repo,
    );

    match service
        .list_by_node(query.node_id, query.entry_category)
        .await
    {
        Ok(list) => Ok((StatusCode::OK, Json(list))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn list_by_scenario(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(scenario_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entry_repo = PlEntryRepositoryImpl::new(state.pool.clone());
    let node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let history_repo = PlEntryHistoryRepositoryImpl::new(state.pool.clone());
    let scenario_repo = ScenarioRepositoryImpl::new(state.pool.clone());

    let service = PlEntryService::new(
        state.pool.clone(),
        entry_repo,
        node_repo,
        history_repo,
        scenario_repo,
    );

    match service.list_by_scenario(scenario_id).await {
        Ok(entries) => Ok((StatusCode::OK, Json(entries))),
        Err(e) => {
            tracing::error!("List entries by scenario error: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}
