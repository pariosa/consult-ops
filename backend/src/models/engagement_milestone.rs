use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EngagementMilestone {
    pub id: i64,
    pub engagement_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub amount_cents: i64,
    pub due_date: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateEngagementMilestone {
    pub engagement_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub amount_cents: i64,
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEngagementMilestoneRequest {
    pub title: String,
    pub description: Option<String>,
    pub amount_cents: Option<i64>,
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEngagementMilestoneRequest {
    pub title: String,
    pub description: Option<String>,
    pub amount_cents: Option<i64>,
    pub due_date: Option<String>,
}

impl EngagementMilestone {
    pub async fn create(db: &PgPool, input: CreateEngagementMilestone) -> SqlxResult<Self> {
        sqlx::query_as::<_, EngagementMilestone>(
            r#"
            INSERT INTO engagement_milestones (
                engagement_id,
                title,
                description,
                amount_cents,
                due_date,
                status,
                created_at,
                updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5,
                'pending',
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP
            )
            RETURNING *
            "#,
        )
        .bind(input.engagement_id)
        .bind(input.title)
        .bind(input.description)
        .bind(input.amount_cents)
        .bind(input.due_date)
        .fetch_one(db)
        .await
    }

    pub async fn for_engagement(db: &PgPool, engagement_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, EngagementMilestone>(
            r#"
            SELECT *
            FROM engagement_milestones
            WHERE engagement_id = $1
            ORDER BY id ASC
            "#,
        )
        .bind(engagement_id)
        .fetch_all(db)
        .await
    }

    pub async fn update_status(db: &PgPool, id: i64, status: &str) -> SqlxResult<Self> {
        sqlx::query_as::<_, EngagementMilestone>(
            r#"
            UPDATE engagement_milestones
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

    pub async fn update(
        db: &PgPool,
        id: i64,
        input: UpdateEngagementMilestoneRequest,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, EngagementMilestone>(
            r#"
            UPDATE engagement_milestones
            SET
                title = $1,
                description = $2,
                amount_cents = $3,
                due_date = $4,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $5
            RETURNING *
            "#,
        )
        .bind(input.title)
        .bind(input.description)
        .bind(input.amount_cents.unwrap_or(0))
        .bind(input.due_date)
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn reopen(db: &PgPool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, EngagementMilestone>(
            r#"
            UPDATE engagement_milestones
            SET
                status = 'pending',
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(db)
        .await
    }
}
