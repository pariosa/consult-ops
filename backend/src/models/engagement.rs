use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result as SqlxResult};

use crate::db::Db;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Engagement {
    pub id: i64,
    pub organization_id: i64,
    pub project_id: i64,
    pub engagement_type: String,
    pub contractor_name: String,
    pub contractor_email: String,
    pub role: String,
    pub title: String,
    pub scope_of_work: String,
    pub deliverables: Option<String>,
    pub repo_url: Option<String>,
    pub amount_cents: i64,
    pub currency: String,
    pub due_date: Option<String>,
    pub status: String,
    pub platform_fee_status: String,
    pub contract_id: Option<i64>,
    pub invoice_id: Option<i64>,
    pub payment_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateEngagement {
    pub organization_id: i64,
    pub project_id: i64,
    pub engagement_type: String,
    pub contractor_name: String,
    pub contractor_email: String,
    pub role: String,
    pub title: String,
    pub scope_of_work: String,
    pub deliverables: Option<String>,
    pub repo_url: Option<String>,
    pub amount_cents: i64,
    pub currency: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEngagementRequest {
    pub engagement_type: String,
    pub contractor_name: String,
    pub contractor_email: String,
    pub role: String,
    pub title: String,
    pub scope_of_work: String,
    pub deliverables: Option<String>,
    pub repo_url: Option<String>,
    pub amount_cents: i64,
    pub currency: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEngagement {
    pub contractor_name: String,
    pub contractor_email: String,
    pub role: String,
    pub title: String,
    pub scope_of_work: String,
    pub deliverables: Option<String>,
    pub repo_url: Option<String>,
    pub amount_cents: i64,
    pub currency: Option<String>,
    pub due_date: Option<String>,
}

impl Engagement {
    pub async fn create(db: &PgPool, input: CreateEngagement) -> SqlxResult<Self> {
        let currency = input.currency.unwrap_or_else(|| "usd".to_string());

        sqlx::query_as::<_, Engagement>(
            r#"
            INSERT INTO engagements (
                organization_id,
                project_id,
                engagement_type,
                contractor_name,
                contractor_email,
                role,
                title,
                scope_of_work,
                deliverables,
                repo_url,
                amount_cents,
                currency,
                due_date,
                status,
                platform_fee_status,
                created_at,
                updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8,
                $9, $10, $11, $12, $13,
                'draft',
                'pending',
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP
            )
            RETURNING *
            "#,
        )
        .bind(input.organization_id)
        .bind(input.project_id)
        .bind(input.engagement_type)
        .bind(input.contractor_name)
        .bind(input.contractor_email)
        .bind(input.role)
        .bind(input.title)
        .bind(input.scope_of_work)
        .bind(input.deliverables)
        .bind(input.repo_url)
        .bind(input.amount_cents)
        .bind(currency)
        .bind(input.due_date)
        .fetch_one(db)
        .await
    }

    pub async fn find(db: &PgPool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, Engagement>(
            r#"
            SELECT *
            FROM engagements
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn find_for_user(db: &PgPool, id: i64, user_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, Engagement>(
            r#"
            SELECT e.*
            FROM engagements e
            JOIN organization_members om
              ON om.organization_id = e.organization_id
            WHERE e.id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(db)
        .await
    }

    pub async fn for_project(db: &PgPool, project_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Engagement>(
            r#"
            SELECT *
            FROM engagements
            WHERE project_id = $1
            ORDER BY id DESC
            "#,
        )
        .bind(project_id)
        .fetch_all(db)
        .await
    }

    pub async fn for_project_for_user(
        db: &PgPool,
        project_id: i64,
        user_id: i64,
    ) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Engagement>(
            r#"
            SELECT e.*
            FROM engagements e
            JOIN organization_members om
              ON om.organization_id = e.organization_id
            WHERE e.project_id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            ORDER BY e.id DESC
            "#,
        )
        .bind(project_id)
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn for_organization(db: &PgPool, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Engagement>(
            r#"
            SELECT *
            FROM engagements
            WHERE organization_id = $1
            ORDER BY id DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(db)
        .await
    }

    pub async fn update(db: &PgPool, id: i64, input: UpdateEngagement) -> SqlxResult<Self> {
        let currency = input.currency.unwrap_or_else(|| "usd".to_string());

        sqlx::query_as::<_, Engagement>(
            r#"
            UPDATE engagements
            SET
                contractor_name = $1,
                contractor_email = $2,
                role = $3,
                title = $4,
                scope_of_work = $5,
                deliverables = $6,
                repo_url = $7,
                amount_cents = $8,
                currency = $9,
                due_date = $10,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $11
            RETURNING *
            "#,
        )
        .bind(input.contractor_name)
        .bind(input.contractor_email)
        .bind(input.role)
        .bind(input.title)
        .bind(input.scope_of_work)
        .bind(input.deliverables)
        .bind(input.repo_url)
        .bind(input.amount_cents)
        .bind(currency)
        .bind(input.due_date)
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn update_status(db: &PgPool, id: i64, status: &str) -> SqlxResult<Self> {
        sqlx::query_as::<_, Engagement>(
            r#"
            UPDATE engagements
            SET
                status = $1,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(status)
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn mark_platform_fee_paid(db: &PgPool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, Engagement>(
            r#"
            UPDATE engagements
            SET
                platform_fee_status = 'paid',
                status = 'active',
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(db)
        .await
    }
    async fn engagement_email_context(
        db: &Db,
        engagement_id: i64,
    ) -> Result<(String, String), sqlx::Error> {
        sqlx::query_as::<_, (String, String)>(
            r#"
        SELECT contractor_email, title
        FROM engagements
        WHERE id = $1
        "#,
        )
        .bind(engagement_id)
        .fetch_one(db.pool.as_ref())
        .await
    }
}
