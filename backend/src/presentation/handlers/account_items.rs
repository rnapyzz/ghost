use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    application::services::account_items::AccountItemService,
    domain::user::UserRole,
    infrastructure::persistence::account_item::AccountItemRepositoryImpl,
    presentation::{dtos::CreateAccountItemRequest, extractors::AuthUser},
    state::AppState,
};

pub async fn create(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateAccountItemRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 権限チェック
    if auth_user.role != UserRole::Admin {
        return Err((StatusCode::FORBIDDEN, "Permission denied".to_string()));
    }

    // バリデーション
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    // サービス実行
    let repo = AccountItemRepositoryImpl::new(state.pool);
    let service = AccountItemService::new(repo);

    match service
        .create(
            payload.name,
            payload.code,
            payload.description,
            payload.account_type,
            payload.display_order,
        )
        .await
    {
        Ok(item) => Ok((StatusCode::CREATED, Json(item))),
        Err(e) => {
            tracing::error!("Failed to create account item: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn list(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let repo = AccountItemRepositoryImpl::new(state.pool);
    let service = AccountItemService::new(repo);

    match service.list_all().await {
        Ok(items) => Ok((StatusCode::OK, Json(items))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
