use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Party {
    pub id: i64,
    pub organization_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub party_type: String,
    pub linked_user_id: Option<i64>,
    pub linked_client_id: Option<i64>,
    pub linked_organization_id: Option<i64>,
    pub is_verified: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateParty {
    pub name: String,
    pub email: Option<String>,
    pub party_type: String,
    pub linked_user_id: Option<i64>,
    pub linked_client_id: Option<i64>,
    pub linked_organization_id: Option<i64>,
    pub is_verified: Option<i64>,
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
                linked_user_id,
                linked_client_id,
                linked_organization_id,
                is_verified,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))
            RETURNING * 
            "#,
        )
        .bind(organization_id)
        .bind(payload.name)
        .bind(payload.email)
        .bind(payload.party_type)
        .bind(payload.linked_user_id)
        .bind(payload.linked_client_id)
        .bind(payload.linked_organization_id)
        .bind(payload.is_verified.unwrap_or(0))
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
    pub async fn create_verified_user_party(
        db: &SqlitePool,
        organization_id: i64,
        user_id: i64,
        party_type: &str,
    ) -> SqlxResult<Self> {
        if let Some(existing) = sqlx::query_as::<_, Party>(
            r#"
        SELECT *
        FROM parties
        WHERE organization_id = ?
          AND linked_user_id = ?
          AND party_type = ?
        LIMIT 1
        "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .bind(party_type)
        .fetch_optional(db)
        .await?
        {
            return Ok(existing);
        }

        sqlx::query_as::<_, Party>(
            r#"
        INSERT INTO parties (
            organization_id,
            name,
            email,
            party_type,
            linked_user_id,
            linked_client_id,
            linked_organization_id,
            is_verified,
            created_at
        )
        SELECT
            ?,
            COALESCE(name, email),
            email,
            ?,
            id,
            NULL,
            ?,
            1,
            datetime('now')
        FROM users
        WHERE id = ?
        RETURNING *
        "#,
        )
        .bind(organization_id)
        .bind(party_type)
        .bind(organization_id)
        .bind(user_id)
        .fetch_one(db)
        .await
    }
    pub async fn create_verified_client_party(
        db: &SqlitePool,
        organization_id: i64,
        client_id: i64,
    ) -> SqlxResult<Self> {
        if let Some(existing) = sqlx::query_as::<_, Party>(
            r#"
        SELECT *
        FROM parties
        WHERE organization_id = ?
          AND linked_client_id = ?
          AND party_type = 'client'
        LIMIT 1
        "#,
        )
        .bind(organization_id)
        .bind(client_id)
        .fetch_optional(db)
        .await?
        {
            return Ok(existing);
        }

        sqlx::query_as::<_, Party>(
            r#"
        INSERT INTO parties (
            organization_id,
            name,
            email,
            party_type,
            linked_user_id,
            linked_client_id,
            linked_organization_id,
            is_verified,
            created_at
        )
        SELECT
            organization_id,
            COALESCE(company_name, name),
            email,
            'client',
            NULL,
            id,
            NULL,
            1,
            datetime('now')
        FROM clients
        WHERE id = ?
          AND organization_id = ?
        RETURNING *
        "#,
        )
        .bind(client_id)
        .bind(organization_id)
        .fetch_one(db)
        .await
    }
}
