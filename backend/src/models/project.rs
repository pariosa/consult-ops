use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Project {
    pub id: i64,
    pub organization_id: i64,
    pub client_id: i64,
    pub name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub client_id: i64,
    pub name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProject {
    pub organization_id: i64,
    pub client_id: i64,
    pub name: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub description: Option<String>,
}

impl Project {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Project>(
            r#"
            SELECT *
            FROM projects
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&*db.pool)
        .await
    }

    pub async fn find(db: &Db, id: i64) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, Project>(
            r#"
            SELECT *
            FROM projects
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*db.pool)
        .await
    }

    pub async fn find_for_user(db: &Db, project_id: i64, user_id: i64) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, Project>(
            r#"
            SELECT p.*
            FROM projects p
            JOIN organization_members om
              ON om.organization_id = p.organization_id
            WHERE p.id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            "#,
        )
        .bind(project_id)
        .bind(user_id)
        .fetch_optional(&*db.pool)
        .await
    }

    pub async fn for_organization(db: &Db, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Project>(
            r#"
            SELECT *
            FROM projects
            WHERE organization_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(&*db.pool)
        .await
    }

    pub async fn create(db: &Db, input: CreateProject) -> SqlxResult<Self> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query_as::<_, Project>(
            r#"
            INSERT INTO projects (
                organization_id,
                client_id,
                name,
                start_date,
                end_date,
                description,
                created_at
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7
            )
            RETURNING *
            "#,
        )
        .bind(input.organization_id)
        .bind(input.client_id)
        .bind(&input.name)
        .bind(&input.start_date)
        .bind(&input.end_date)
        .bind(&input.description)
        .bind(&now)
        .fetch_one(&*db.pool)
        .await
    }

    pub async fn update(
        db: &Db,
        project_id: i64,
        organization_id: i64,
        input: CreateProjectRequest,
    ) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, Project>(
            r#"
            UPDATE projects
            SET
                client_id = $1,
                name = $2,
                start_date = $3,
                end_date = $4,
                description = $5
            WHERE id = $6
              AND organization_id = $7
            RETURNING *
            "#,
        )
        .bind(input.client_id)
        .bind(input.name)
        .bind(input.start_date)
        .bind(input.end_date)
        .bind(input.description)
        .bind(project_id)
        .bind(organization_id)
        .fetch_optional(&*db.pool)
        .await
    }

    pub async fn delete(db: &Db, project_id: i64, organization_id: i64) -> SqlxResult<bool> {
        let result = sqlx::query(
            r#"
            DELETE FROM projects
            WHERE id = $1
              AND organization_id = $2
            "#,
        )
        .bind(project_id)
        .bind(organization_id)
        .execute(&*db.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
