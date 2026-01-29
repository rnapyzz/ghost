use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{account_items::AccountType, plan_nodes::NodeType, user::UserRole};

#[derive(Debug, Deserialize, Validate)]
pub struct SignUpRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAccountItemRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(length(min = 1, message = "Code is required"))]
    pub code: String,

    pub description: Option<String>,
    pub account_type: AccountType,
    pub display_order: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateServiceRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(length(min = 1, message = "Slug is required"))]
    pub slug: String,

    pub display_order: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateScenarioRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    pub description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePlanNodeRequest {
    pub scenario_id: Uuid,

    // Rootの場合はnullで送られてくる
    pub parent_id: Option<Uuid>,

    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,

    pub description: Option<String>,
    pub node_type: NodeType,
    pub display_order: i32,

    // 実体タイプ（Job, Buffer）の場合のみ指定
    pub service_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ListPlanNodesQuery {
    pub scenario_id: Uuid,
}
