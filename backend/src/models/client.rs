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
        sqlx::query_as::<_, Client>("SELECT * FROM clients")
            .fetch_all(&*db.pool)
            .await
    }

    pub async fn create(db: &Db, client: CreateClient) -> SqlxResult<Self> {
        let rec: sqlx::sqlite::SqliteQueryResult = sqlx::query(
            r#"
                INSERT INTO clients 
                (organization_id, name, email, phone, company_name, address, city, state, zip, country)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(client.organization_id)
        .bind(&client.name)
        .bind(&client.email)
        .bind(&client.phone)
        .bind(&client.company_name)
        .bind(&client.address)
        .bind(&client.city)
        .bind(&client.state)
        .bind(&client.zip)
        .bind(&client.country)
        .execute(&*db.pool)
        .await?;

        Ok(Client {
            id: rec.last_insert_rowid(),
            organization_id: client.organization_id,
            name: client.name,
            email: client.email,
            tax_id: client.tax_id,
            phone: client.phone,
            company_name: client.company_name,
            address: client.address,
            city: client.city,
            state: client.state,
            zip: client.zip,
            country: client.country,
            created_at: None,
            updated_at: None,
        })
    }
}
