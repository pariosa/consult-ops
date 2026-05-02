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

#[derive(Deserialize)]
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
        sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY created_at DESC")
            .fetch_all(&*db.pool)
            .await
    }

    pub async fn create(db: &Db, input: CreateProject) -> SqlxResult<Self> {
        let now = chrono::Utc::now().to_rfc3339();
        let rec = sqlx::query(
            r#"
                INSERT INTO projects
                (organization_id, client_id, name, start_date, end_date, description, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(input.organization_id)
        .bind(input.client_id)
        .bind(&input.name)
        .bind(&input.start_date)
        .bind(&input.end_date)
        .bind(&input.description)
        .bind(&now)
        .execute(&*db.pool)
        .await?;

        Ok(Project {
            id: rec.last_insert_rowid(),
            organization_id: input.organization_id,
            client_id: input.client_id,
            name: input.name,
            start_date: input.start_date,
            end_date: input.end_date,
            description: input.description,
            created_at: now,
        })
    }
}
