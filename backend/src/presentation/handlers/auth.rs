use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    application::services::auth::AuthService,
    infrastructure::persistence::user::UserRepositoryImpl,
    presentation::dtos::{SignUpRequest, UserResponse},
    state::AppState,
};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignUpRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Validation error: {}", e)));
    }

    let repo = UserRepositoryImpl::new(state.pool);
    let service = AuthService::new(repo);

    match service
        .signup(payload.name, payload.email, payload.password)
        .await
    {
        Ok(user) => {
            let response = UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                role: user.role,
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("Email already exists") {
                Err((StatusCode::CONFLICT, msg))
            } else {
                tracing::error!("Signup failed: {:?}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                ))
            }
        }
    }
}
