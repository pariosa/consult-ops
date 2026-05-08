use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Engagement {
    pub id: i64,
    pub organization_id: i64,
    pub project_id: i64,
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
    pub async fn create(db: &SqlitePool, input: CreateEngagement) -> SqlxResult<Self> {
        let currency = input.currency.unwrap_or_else(|| "usd".to_string());

        let rec = sqlx::query_as::<_, Engagement>(
            r#"
            INSERT INTO engagements (
                organization_id,
                project_id,
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
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'draft', 'pending', datetime('now'))
            RETURNING *
            "#,
        )
        .bind(input.organization_id)
        .bind(input.project_id)
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
        .await?;

        Ok(rec)
    }

    pub async fn find(db: &SqlitePool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, Engagement>("SELECT * FROM engagements WHERE id = ?")
            .bind(id)
            .fetch_one(db)
            .await
    }

    pub async fn for_project(db: &SqlitePool, project_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Engagement>(
            "SELECT * FROM engagements WHERE project_id = ? ORDER BY id DESC",
        )
        .bind(project_id)
        .fetch_all(db)
        .await
    }

    pub async fn update_status(db: &SqlitePool, id: i64, status: &str) -> SqlxResult<Self> {
        sqlx::query_as::<_, Engagement>(
            r#"
            UPDATE engagements
            SET status = ?
            WHERE id = ?
            RETURNING *
            "#,
        )
        .bind(status)
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn mark_platform_fee_paid(db: &SqlitePool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, Engagement>(
            r#"
            UPDATE engagements
            SET platform_fee_status = 'paid',
                status = 'active'
            WHERE id = ?
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(db)
        .await
    }
}
