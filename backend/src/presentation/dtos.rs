use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    account_items::AccountType, pl_entries::EntryCategory, plan_nodes::NodeType, user::UserRole,
};

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
pub struct UpdatePlanNodeRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ListPlanNodesQuery {
    pub scenario_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SavePlEntryRequest {
    pub node_id: Uuid,
    pub account_item_id: Uuid,
    pub target_month: NaiveDate, // YYYY-MM-01
    pub entry_category: EntryCategory,
    pub amount: Decimal,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct BulkSavePlEntryRequest {
    #[validate(nested)]
    pub entries: Vec<SavePlEntryRequest>,
}

#[derive(Debug, Deserialize)]
pub struct ListPlEntryQuery {
    pub node_id: Uuid,
    pub entry_category: EntryCategory,
}
