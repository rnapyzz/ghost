use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::user::UserRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: UserRole,
    pub exp: usize,
    pub iat: usize,
}

pub fn generate_token(user_id: Uuid, role: UserRole) -> anyhow::Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(72))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        role,
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_TOKEN must be set");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}
