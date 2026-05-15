// src/seed_demo.rs

use crate::auth::hash_password;
use crate::db::Db;
use crate::models::client::CreateClient;
use crate::models::contract::CreateContract;
use crate::models::invoice::CreateInvoice;
use crate::models::project::CreateProject;
use crate::models::{Client, Contract, Invoice, Payment, Project};

use chrono::{Duration, Utc};

pub async fn seed_demo_data(db: &Db) -> sqlx::Result<()> {
    let now = Utc::now().to_rfc3339();
    let demo_password = hash_password("DemoPass123!").expect("Failed to hash demo password");

    create_platform_user(
        db,
        "superadmin@consultops.test",
        "Platform Super Admin",
        "super_admin",
        &demo_password,
    )
    .await?;
    // =====================================================
    // ORG 1
    // Atlas Field Consulting
    // Municipal / Infrastructure / Field Ops
    // =====================================================

    let atlas_org_id = get_or_create_org(db, "Atlas Field Consulting").await?;

    create_user_and_membership(
        db,
        "owner@atlas.test",
        "Olivia Owner",
        "owner",
        "owner",
        atlas_org_id,
        &demo_password,
    )
    .await?;

    create_user_and_membership(
        db,
        "admin@atlas.test",
        "Avery Atlas",
        "admin",
        "admin",
        atlas_org_id,
        &demo_password,
    )
    .await?;

    create_user_and_membership(
        db,
        "finance@atlas.test",
        "Finley Finance",
        "finance_admin",
        "finance_admin",
        atlas_org_id,
        &demo_password,
    )
    .await?;

    create_user_and_membership(
        db,
        "ops@atlas.test",
        "Riley Operations",
        "operations_manager",
        "operations_manager",
        atlas_org_id,
        &demo_password,
    )
    .await?;

    create_user_and_membership(
        db,
        "contractor@atlas.test",
        "Jordan Contractor",
        "contractor",
        "contractor",
        atlas_org_id,
        &demo_password,
    )
    .await?;

    create_user_and_membership(
        db,
        "client.viewer@atlas.test",
        "Morgan Client Viewer",
        "client_viewer",
        "client_viewer",
        atlas_org_id,
        &demo_password,
    )
    .await?;

    let atlas_client = create_client_if_missing(
        db,
        CreateClient {
            organization_id: atlas_org_id,
            name: "Riverbend Municipal Water Authority".to_string(),
            email: "ops@riverbend.gov".to_string(),
            tax_id: Some("RBWA-7781".to_string()),
            phone: Some("555-1100".to_string()),
            company_name: Some("Riverbend Municipal Water Authority".to_string()),
            address: Some("18 Reservoir Road".to_string()),
            city: Some("Riverbend".to_string()),
            state: Some("Ohio".to_string()),
            zip: Some("45110".to_string()),
            country: Some("United States".to_string()),
        },
    )
    .await?;

    let atlas_project = create_project_if_missing(
        db,
        CreateProject {
            organization_id: atlas_org_id,
            client_id: atlas_client.id,
            name: "Pump Station Modernization".to_string(),
            start_date: Some(now.clone()),
            description: Some(
                "Upgrade SCADA systems, replace valves, improve telemetry.".to_string(),
            ),
            end_date: Some((Utc::now() + Duration::days(180)).to_rfc3339()),
        },
    )
    .await?;

    let atlas_contract = create_contract_if_missing(
        db,
        CreateContract {
            organization_id: atlas_org_id,
            project_id: atlas_project.id,
            title: "Infrastructure Delivery Agreement".to_string(),
            status: "Active".to_string(),
            signed_at: Some(now.clone()),
            start_date: Some(now.clone()),
            end_date: Some((Utc::now() + Duration::days(365)).to_rfc3339()),
            value: Some(185000.0),
            currency: Some("USD".to_string()),
            terms: Some("Milestone billing tied to site completion phases.".to_string()),
            notes: Some("Public works contract".to_string()),
            external_id: Some("ATLAS-INFRA-001".to_string()),
            created_at: now.clone(),
        },
    )
    .await?;

    let atlas_invoice = create_invoice_if_missing(
        db,
        CreateInvoice {
            organization_id: atlas_org_id,
            contract_id: atlas_contract.id,
            invoice_number: "ATLAS-INV-001".to_string(),
            status: "Pending".to_string(),
            issued_at: Some(now.clone()),
            due_date: Some((Utc::now() + Duration::days(30)).to_rfc3339()),
            subtotal: Some(50000.0),
            tax: Some(0.0),
            total: Some(50000.0),
            currency: Some("USD".to_string()),
            notes: Some("Milestone 1: Site survey and controls replacement.".to_string()),
            created_at: now.clone(),
        },
    )
    .await?;

    create_payment_if_missing(
        db,
        atlas_org_id,
        atlas_invoice.id,
        50000.0,
        "USD",
        "ACH",
        "ATLAS-PAY-001",
        "Municipal ACH milestone payment",
    )
    .await?;

    // =====================================================
    // ORG 2
    // Verdant Retail Systems
    // Retail / Ecommerce / Inventory
    // =====================================================

    let verdant_org_id = get_or_create_org(db, "Verdant Retail Systems").await?;

    create_user_and_membership(
        db,
        "owner@verdant.test",
        "Oliver Ownerton",
        "owner",
        "owner",
        verdant_org_id,
        &demo_password,
    )
    .await?;

    create_user_and_membership(
        db,
        "admin@verdant.test",
        "Vincent Vadmin",
        "admin",
        "admin",
        verdant_org_id,
        &demo_password,
    )
    .await?;

    create_user_and_membership(
        db,
        "finance@verdant.test",
        "Franklin Financier",
        "finance_admin",
        "finance_admin",
        verdant_org_id,
        &demo_password,
    )
    .await?;

    create_user_and_membership(
        db,
        "ops@verdant.test",
        "Binksy Operations",
        "operations_manager",
        "operations_manager",
        verdant_org_id,
        &demo_password,
    )
    .await?;

    create_user_and_membership(
        db,
        "contractor@verdant.test",
        "Herbie Contractor",
        "contractor",
        "contractor",
        verdant_org_id,
        &demo_password,
    )
    .await?;

    create_user_and_membership(
        db,
        "client.viewer@verdant.test",
        "Clemson Client",
        "client_viewer",
        "client_viewer",
        verdant_org_id,
        &demo_password,
    )
    .await?;

    let verdant_client = create_client_if_missing(
        db,
        CreateClient {
            organization_id: verdant_org_id,
            name: "Moss & Market Cooperative".to_string(),
            email: "hello@mossmarket.co".to_string(),
            tax_id: Some("MMC-9911".to_string()),
            phone: Some("555-2200".to_string()),
            company_name: Some("Moss & Market Cooperative".to_string()),
            address: Some("42 Cedar Lane".to_string()),
            city: Some("Portland".to_string()),
            state: Some("Oregon".to_string()),
            zip: Some("97201".to_string()),
            country: Some("United States".to_string()),
        },
    )
    .await?;

    let verdant_project = create_project_if_missing(
        db,
        CreateProject {
            organization_id: verdant_org_id,
            client_id: verdant_client.id,
            name: "Omnichannel Inventory Rollout".to_string(),
            start_date: Some(now.clone()),
            description: Some(
                "POS sync, ecommerce inventory automation, reorder alerts.".to_string(),
            ),
            end_date: Some((Utc::now() + Duration::days(120)).to_rfc3339()),
        },
    )
    .await?;

    let verdant_contract = create_contract_if_missing(
        db,
        CreateContract {
            organization_id: verdant_org_id,
            project_id: verdant_project.id,
            title: "Retail Systems Transformation Agreement".to_string(),
            status: "Active".to_string(),
            signed_at: Some(now.clone()),
            start_date: Some(now.clone()),
            end_date: Some((Utc::now() + Duration::days(240)).to_rfc3339()),
            value: Some(62000.0),
            currency: Some("USD".to_string()),
            terms: Some("Monthly retainers + go-live bonus structure.".to_string()),
            notes: Some("Retail modernization engagement".to_string()),
            external_id: Some("VERDANT-RET-001".to_string()),
            created_at: now.clone(),
        },
    )
    .await?;

    let verdant_invoice = create_invoice_if_missing(
        db,
        CreateInvoice {
            organization_id: verdant_org_id,
            contract_id: verdant_contract.id,
            invoice_number: "VERDANT-INV-001".to_string(),
            status: "Paid".to_string(),
            issued_at: Some(now.clone()),
            due_date: Some((Utc::now() + Duration::days(14)).to_rfc3339()),
            subtotal: Some(12500.0),
            tax: Some(875.0),
            total: Some(13375.0),
            currency: Some("USD".to_string()),
            notes: Some("Phase 1 ecommerce catalog sync completed.".to_string()),
            created_at: now.clone(),
        },
    )
    .await?;

    create_payment_if_missing(
        db,
        verdant_org_id,
        verdant_invoice.id,
        13375.0,
        "USD",
        "Card",
        "VERDANT-PAY-001",
        "Corporate card payment",
    )
    .await?;

    Ok(())
}

// =====================================================
// HELPERS
// =====================================================

async fn get_or_create_org(db: &Db, name: &str) -> sqlx::Result<i64> {
    if let Some(id) =
        sqlx::query_scalar::<_, i64>("SELECT id FROM organizations WHERE name = ? LIMIT 1")
            .bind(name)
            .fetch_optional(&*db.pool)
            .await?
    {
        return Ok(id);
    }

    let now = Utc::now().to_rfc3339();

    let rec = sqlx::query(
        r#"
        INSERT INTO organizations (name, created_at, updated_at)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(name)
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await?;

    Ok(rec.last_insert_rowid())
}

async fn create_user_and_membership(
    db: &Db,
    email: &str,
    name: &str,
    user_type: &str,
    role: &str,
    organization_id: i64,
    password_hash: &str,
) -> sqlx::Result<()> {
    let user_id = if let Some(id) =
        sqlx::query_scalar::<_, i64>("SELECT id FROM users WHERE email = ? LIMIT 1")
            .bind(email)
            .fetch_optional(&*db.pool)
            .await?
    {
        id
    } else {
        let now = Utc::now().to_rfc3339();

        let rec = sqlx::query(
            r#"
            INSERT INTO users
            (email, password_hash, name, user_type, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(email)
        .bind(password_hash)
        .bind(name)
        .bind(user_type)
        .bind(&now)
        .bind(&now)
        .execute(&*db.pool)
        .await?;

        rec.last_insert_rowid()
    };

    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO organization_members
            (organization_id, user_id, role, status, created_at, updated_at)
        VALUES (?, ?, ?, 'active', ?, ?)
        ON CONFLICT(organization_id, user_id)
        DO UPDATE SET
        role = excluded.role,
        status = 'active',
        updated_at = excluded.updated_at
        "#,
    )
    .bind(organization_id)
    .bind(user_id)
    .bind(role)
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await?;

    Ok(())
}

async fn create_client_if_missing(db: &Db, payload: CreateClient) -> sqlx::Result<Client> {
    let all = Client::all(db).await?;

    if let Some(existing) = all
        .into_iter()
        .find(|c| c.organization_id == payload.organization_id && c.name == payload.name)
    {
        return Ok(existing);
    }

    Client::create(db, payload).await
}

async fn create_project_if_missing(db: &Db, payload: CreateProject) -> sqlx::Result<Project> {
    let all = Project::all(db).await?;

    if let Some(existing) = all
        .into_iter()
        .find(|p| p.organization_id == payload.organization_id && p.name == payload.name)
    {
        return Ok(existing);
    }

    Project::create(db, payload).await
}

async fn create_contract_if_missing(db: &Db, payload: CreateContract) -> sqlx::Result<Contract> {
    let all = Contract::all(db).await?;

    if let Some(existing) = all
        .into_iter()
        .find(|c| c.organization_id == payload.organization_id && c.title == payload.title)
    {
        return Ok(existing);
    }

    Contract::create(db, payload).await
}

async fn create_invoice_if_missing(db: &Db, payload: CreateInvoice) -> sqlx::Result<Invoice> {
    let all = Invoice::all(db).await?;

    if let Some(existing) = all.into_iter().find(|i| {
        i.organization_id == payload.organization_id && i.invoice_number == payload.invoice_number
    }) {
        return Ok(existing);
    }

    Invoice::create(db, payload).await
}

async fn create_payment_if_missing(
    db: &Db,
    organization_id: i64,
    invoice_id: i64,
    amount: f64,
    currency: &str,
    method: &str,
    reference: &str,
    notes: &str,
) -> sqlx::Result<()> {
    let payments = Payment::all(db).await?;

    if payments
        .iter()
        .any(|p| p.reference.as_deref() == Some(reference))
    {
        return Ok(());
    }

    Payment::create(
        db,
        Payment {
            id: 0,
            organization_id,
            invoice_id,
            paid_at: Some(Utc::now().to_rfc3339()),
            amount,
            currency: Some(currency.to_string()),
            method: Some(method.to_string()),
            reference: Some(reference.to_string()),
            notes: Some(notes.to_string()),
            created_at: Utc::now().to_rfc3339(),
        },
    )
    .await?;

    Ok(())
}
async fn create_platform_user(
    db: &Db,
    email: &str,
    name: &str,
    user_type: &str,
    password_hash: &str,
) -> sqlx::Result<i64> {
    let now = Utc::now().to_rfc3339();

    if let Some(id) = sqlx::query_scalar::<_, i64>("SELECT id FROM users WHERE email = ? LIMIT 1")
        .bind(email)
        .fetch_optional(&*db.pool)
        .await?
    {
        sqlx::query(
            r#"
            UPDATE users
            SET name = ?, user_type = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(name)
        .bind(user_type)
        .bind(&now)
        .bind(id)
        .execute(&*db.pool)
        .await?;

        return Ok(id);
    }

    let rec = sqlx::query(
        r#"
        INSERT INTO users
        (email, password_hash, name, user_type, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(name)
    .bind(user_type)
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await?;

    Ok(rec.last_insert_rowid())
}
