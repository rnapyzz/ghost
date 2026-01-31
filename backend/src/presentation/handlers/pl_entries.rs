use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use rust_decimal::Decimal;
use validator::Validate;

use crate::{
    application::services::pl_entries::PlEntryService,
    infrastructure::persistence::{
        history::PlEntryHistoryRepositoryImpl, pl_entries::PlEntryRepositoryImpl,
        plan_nodes::PlanNodeRepositoryImpl,
    },
    presentation::{
        dtos::{ListPlEntryQuery, SavePlEnetryRequest},
        extractors::AuthUser,
    },
    state::AppState,
};

pub async fn save(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<SavePlEnetryRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let entry_repo = PlEntryRepositoryImpl::new(state.pool.clone());
    let node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let history_repo = PlEntryHistoryRepositoryImpl::new(state.pool.clone());
    let service = PlEntryService::new(entry_repo, node_repo, history_repo);

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

pub async fn list(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Query(query): Query<ListPlEntryQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entry_repo = PlEntryRepositoryImpl::new(state.pool.clone());
    let node_repo = PlanNodeRepositoryImpl::new(state.pool.clone());
    let history_repo = PlEntryHistoryRepositoryImpl::new(state.pool.clone());
    let service = PlEntryService::new(entry_repo, node_repo, history_repo);

    match service
        .list_by_node(query.node_id, query.entry_category)
        .await
    {
        Ok(list) => Ok((StatusCode::OK, Json(list))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
