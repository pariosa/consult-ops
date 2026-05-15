use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Party {
    pub id: i64,
    pub organization_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub party_type: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateParty {
    pub name: String,
    pub email: Option<String>,
    pub party_type: String,
}

impl Party {
    pub async fn create(
        db: &SqlitePool,
        organization_id: i64,
        payload: CreateParty,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, Party>(
            r#"
            INSERT INTO parties (
                organization_id,
                name,
                email,
                party_type,
                created_at
            )
            VALUES (?, ?, ?, ?, datetime('now'))
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(payload.name)
        .bind(payload.email)
        .bind(payload.party_type)
        .fetch_one(db)
        .await
    }

    pub async fn for_organization(db: &SqlitePool, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Party>(
            r#"
            SELECT *
            FROM parties
            WHERE organization_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(db)
        .await
    }
}
