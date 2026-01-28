use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    application::services::services::ServiceService,
    domain::user::UserRole,
    infrastructure::persistence::services::ServiceRepositoryImpl,
    presentation::{dtos::CreateServiceRequest, extractors::AuthUser},
    state::AppState,
};

pub async fn create(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateServiceRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if auth_user.role != UserRole::Admin {
        return Err((StatusCode::FORBIDDEN, "Permission denied".to_string()));
    }

    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let repo = ServiceRepositoryImpl::new(state.pool);
    let service = ServiceService::new(repo);

    match service
        .create(payload.name, payload.slug, payload.display_order)
        .await
    {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("Slug already exists") || msg.contains("Slug must contain") {
                Err((StatusCode::BAD_REQUEST, msg))
            } else {
                tracing::error!("Create service error: {}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, msg))
            }
        }
    }
}

pub async fn list(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let repo = ServiceRepositoryImpl::new(state.pool);
    let service = ServiceService::new(repo);

    match service.list_all().await {
        Ok(rec) => Ok((StatusCode::OK, Json(rec))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
