use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use uuid::Uuid;
use validator::Validate;

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

    let repo = ScenarioRepositoryImpl::new(state.pool);
    let service = ScenarioService::new(repo);

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
    let repo = ScenarioRepositoryImpl::new(state.pool);
    let service = ScenarioService::new(repo);

    match service.list_all().await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn activate(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let repo = ScenarioRepositoryImpl::new(state.pool);
    let service = ScenarioService::new(repo);

    match service.activate(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Error activating scenario: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}
