// seed_demo.rs
use crate::db::Db;
use crate::models::{Client, Project};
use chrono::Utc;

pub async fn seed_demo_data(db: &Db) -> sqlx::Result<()> {
    // seed one client if none exists
    let clients = Client::all(db).await?;
    if clients.is_empty() {
        Client::create(db, Client {
            id: 0,
            name: "Demo Client".to_string(),
            email: "demo@example.com".to_string(),
            phone: Some("555-5555".to_string()),
        }).await?;
    }

    // seed one project
    let projects = Project::all(db).await?;
    if projects.is_empty() {
        let client = Client::all(db).await?.first().unwrap().clone();
        Project::create(db, Project {
            id: 0,
            client_id: client.id,
            name: "Demo Project".to_string(),
            start_date: Utc::now().to_string(),
        }).await?;
    }

    Ok(())
}