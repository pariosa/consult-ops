use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use std::fmt;

use crate::auth_context::AuthUser;

#[derive(Debug, FromRow, Serialize)]
pub struct OrgMembership {
    pub organization_id: i64,
    pub user_id: i64,
    pub role: String,
    pub status: String,
}

#[derive(Debug)]
pub enum AuthzError {
    Forbidden,
    Db(sqlx::Error),
}

impl fmt::Display for AuthzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthzError::Forbidden => write!(f, "Forbidden"),
            AuthzError::Db(e) => write!(f, "{}", e),
        }
    }
}

impl ResponseError for AuthzError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthzError::Forbidden => HttpResponse::Forbidden().json(serde_json::json!({
                "error": "You do not have access to this organization"
            })),
            AuthzError::Db(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}

impl From<sqlx::Error> for AuthzError {
    fn from(e: sqlx::Error) -> Self {
        AuthzError::Db(e)
    }
}

pub fn require_platform_admin(auth: &AuthUser) -> Result<(), AuthzError> {
    if auth.user_type == "admin" || auth.user_type == "super_admin" {
        Ok(())
    } else {
        Err(AuthzError::Forbidden)
    }
}

pub async fn require_org_member(
    pool: &PgPool,
    user_id: i64,
    organization_id: i64,
) -> Result<OrgMembership, AuthzError> {
    let membership = sqlx::query_as::<_, OrgMembership>(
        r#"
        SELECT organization_id, user_id, role, status
        FROM organization_members
        WHERE user_id = $1
          AND organization_id = $2
          AND status = 'active'
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .bind(organization_id)
    .fetch_optional(pool)
    .await?;

    membership.ok_or(AuthzError::Forbidden)
}

pub async fn require_org_role(
    pool: &PgPool,
    user_id: i64,
    organization_id: i64,
    allowed_roles: &[&str],
) -> Result<OrgMembership, AuthzError> {
    let membership = require_org_member(pool, user_id, organization_id).await?;

    if allowed_roles.iter().any(|role| *role == membership.role) {
        Ok(membership)
    } else {
        Err(AuthzError::Forbidden)
    }
}
