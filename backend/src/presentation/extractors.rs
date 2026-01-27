use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

use crate::{domain::user::UserRole, utils::jwt::Claims};

pub struct AuthUser {
    pub id: Uuid,
    pub role: UserRole,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Authorizationヘッダーを取得
        let auth_header = parts.headers.get(AUTHORIZATION).ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing Authorization header".to_string(),
        ))?;

        // 文字列に変換する
        let auth_value = auth_header.to_str().map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header".to_string(),
            )
        })?;

        // "Bearer "プレフィックスのチェックと削除
        if !auth_value.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Invalid token format".to_string()));
        }
        let token = &auth_value[7..];

        // JWTの検証とでコード
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| {
            tracing::error!("JWT validation failed: {}", e);
            (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
        })?;

        Ok(AuthUser {
            id: token_data.claims.sub,
            role: token_data.claims.role,
        })
    }
}
