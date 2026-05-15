use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PermissionError {
    pub error: String,
}

impl std::fmt::Display for PermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl ResponseError for PermissionError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Forbidden().json(serde_json::json!({
            "error": self.error
        }))
    }
}
pub fn can_manage_platform(role: &str) -> bool {
    matches!(role, "super_admin")
}

pub fn can_manage_finance(role: &str) -> bool {
    matches!(role, "super_admin" | "owner" | "admin" | "finance_admin")
}

pub fn can_manage_agreements(role: &str) -> bool {
    matches!(role, "super_admin" | "owner" | "admin" | "finance_admin")
}

pub fn can_manage_transactions(role: &str) -> bool {
    matches!(role, "super_admin" | "owner" | "admin" | "finance_admin")
}

pub fn can_process_transactions(role: &str) -> bool {
    matches!(
        role,
        "super_admin" | "owner" | "admin" | "finance_admin" | "operations_manager"
    )
}

pub fn can_manage_milestones(role: &str) -> bool {
    matches!(
        role,
        "super_admin" | "owner" | "admin" | "finance_admin" | "operations_manager" | "contractor"
    )
}

pub fn require_permission(allowed: bool, message: &str) -> Result<(), PermissionError> {
    if allowed {
        Ok(())
    } else {
        Err(PermissionError {
            error: message.to_string(),
        })
    }
}
