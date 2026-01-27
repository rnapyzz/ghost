use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    domain::user::UserRepository, infrastructure::persistence::user::UserRepositoryImpl,
    presentation::extractors::AuthUser, state::AppState,
};

pub async fn get_me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // auth_userにはidとroleしか入っていないので、idから最新の情報を取得する
    let repo = UserRepositoryImpl::new(state.pool);

    let user = repo
        .find_by_id(auth_user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user))
}
