use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Party {
    pub id: i64,
    pub organization_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub party_type: String,
    pub is_verified: i64,

    pub verification_status: String,
    pub verified_at: Option<String>,
    pub verification_method: Option<String>,

    pub linked_user_id: Option<i64>,
    pub linked_client_id: Option<i64>,
    pub linked_organization_id: Option<i64>,
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
    pub verification_status: Option<String>,
    pub verification_method: Option<String>,
}

impl Party {
    pub async fn create(
        db: &PgPool,
        organization_id: i64,
        payload: CreateParty,
    ) -> SqlxResult<Self> {
        let is_verified = payload.is_verified.unwrap_or(0);

        let verification_status = payload.verification_status.unwrap_or_else(|| {
            if is_verified == 1 {
                "verified".to_string()
            } else {
                "unverified".to_string()
            }
        });

        sqlx::query_as::<_, Party>(
            r#"
            INSERT INTO parties (
                organization_id,
                name,
                email,
                party_type,
                is_verified,
                verification_status,
                verified_at,
                verification_method,
                linked_user_id,
                linked_client_id,
                linked_organization_id,
                created_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6,
                CASE WHEN $7 = 1 THEN CURRENT_TIMESTAMP ELSE NULL END,
                $8, $9, $10, $11, CURRENT_TIMESTAMP
            )
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(payload.name)
        .bind(payload.email)
        .bind(payload.party_type)
        .bind(is_verified)
        .bind(verification_status)
        .bind(is_verified)
        .bind(payload.verification_method)
        .bind(payload.linked_user_id)
        .bind(payload.linked_client_id)
        .bind(payload.linked_organization_id)
        .fetch_one(db)
        .await
    }

    pub async fn for_organization(db: &PgPool, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Party>(
            r#"
            SELECT *
            FROM parties
            WHERE organization_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(db)
        .await
    }

    pub async fn for_organization_for_user(
        db: &PgPool,
        organization_id: i64,
        user_id: i64,
    ) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Party>(
            r#"
            SELECT p.*
            FROM parties p
            JOIN organization_members om
              ON om.organization_id = p.organization_id
            WHERE p.organization_id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            ORDER BY p.created_at DESC
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn find(db: &PgPool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, Party>(
            r#"
            SELECT *
            FROM parties
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn find_for_user(db: &PgPool, id: i64, user_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, Party>(
            r#"
            SELECT p.*
            FROM parties p
            JOIN organization_members om
              ON om.organization_id = p.organization_id
            WHERE p.id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(db)
        .await
    }

    pub async fn create_verified_user_party(
        db: &PgPool,
        organization_id: i64,
        user_id: i64,
        party_type: &str,
    ) -> SqlxResult<Self> {
        if let Some(existing) = sqlx::query_as::<_, Party>(
            r#"
            SELECT *
            FROM parties
            WHERE organization_id = $1
              AND linked_user_id = $2
              AND party_type = $3
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
                verification_status,
                verified_at,
                verification_method,
                created_at
            )
            SELECT
                $1,
                COALESCE(name, email),
                email,
                $2,
                id,
                NULL,
                $3,
                1,
                'verified',
                CURRENT_TIMESTAMP,
                'linked_user',
                CURRENT_TIMESTAMP
            FROM users
            WHERE id = $4
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
        db: &PgPool,
        organization_id: i64,
        client_id: i64,
    ) -> SqlxResult<Self> {
        if let Some(existing) = sqlx::query_as::<_, Party>(
            r#"
            SELECT *
            FROM parties
            WHERE organization_id = $1
              AND linked_client_id = $2
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
                verification_status,
                verified_at,
                verification_method,
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
                'verified',
                CURRENT_TIMESTAMP,
                'linked_client',
                CURRENT_TIMESTAMP
            FROM clients
            WHERE id = $1
              AND organization_id = $2
            RETURNING *
            "#,
        )
        .bind(client_id)
        .bind(organization_id)
        .fetch_one(db)
        .await
    }

    pub async fn verify(db: &PgPool, id: i64, verification_method: &str) -> SqlxResult<Self> {
        sqlx::query_as::<_, Party>(
            r#"
            UPDATE parties
            SET
                is_verified = 1,
                verification_status = 'verified',
                verified_at = CURRENT_TIMESTAMP,
                verification_method = $1
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(verification_method)
        .bind(id)
        .fetch_one(db)
        .await
    }
}
