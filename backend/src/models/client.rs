use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Client {
    pub id: i64,
    pub organization_id: i64,
    pub name: String,
    pub email: String,
    pub tax_id: Option<String>,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateClient {
    pub organization_id: i64,
    pub name: String,
    pub email: String,
    pub tax_id: Option<String>,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,
}

impl Client {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Client>(
            r#"
            SELECT *
            FROM clients
            "#,
        )
        .fetch_all(&*db.pool)
        .await
    }

    pub async fn create(db: &Db, client: CreateClient) -> SqlxResult<Self> {
        sqlx::query_as::<_, Client>(
            r#"
            INSERT INTO clients (
                organization_id,
                name,
                email,
                tax_id,
                phone,
                company_name,
                address,
                city,
                state,
                zip,
                country,
                created_at,
                updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10,
                $11, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
            )
            RETURNING *
            "#,
        )
        .bind(client.organization_id)
        .bind(&client.name)
        .bind(&client.email)
        .bind(&client.tax_id)
        .bind(&client.phone)
        .bind(&client.company_name)
        .bind(&client.address)
        .bind(&client.city)
        .bind(&client.state)
        .bind(&client.zip)
        .bind(&client.country)
        .fetch_one(&*db.pool)
        .await
    }
    pub async fn for_organization(db: &Db, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Client>(
            r#"
        SELECT *
        FROM clients
        WHERE organization_id = $1
        ORDER BY name ASC
        "#,
        )
        .bind(organization_id)
        .fetch_all(db.pool.as_ref())
        .await
    }
}
