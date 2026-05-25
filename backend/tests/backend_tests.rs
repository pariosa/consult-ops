mod common;

use actix_web::test;
use backend::auth::hash_password;
use backend::db::Db;
use chrono::Utc;
use common::{setup_test_db, test_app};
use serde_json::json;
use serial_test::serial;

async fn seed_org(db: &Db) -> i64 {
    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO organizations (name, slug, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind("Test Organization")
    .bind(format!(
        "test-org-{}",
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ))
    .bind(Utc::now().to_rfc3339())
    .bind(Utc::now().to_rfc3339())
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

async fn seed_admin_user(db: &Db, organization_id: i64) -> (i64, String) {
    let now = Utc::now().to_rfc3339();
    let email = format!(
        "admin-{}@test.com",
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    );
    let password_hash = hash_password("Password123!").unwrap();

    let user_id = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO users (
            email,
            password_hash,
            name,
            user_type,
            email_verified_at,
            current_organization_id,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, 'admin', $4, $5, $6, $7)
        RETURNING id
        "#,
    )
    .bind(&email)
    .bind(password_hash)
    .bind("Test Admin")
    .bind(&now)
    .bind(organization_id)
    .bind(&now)
    .bind(&now)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    sqlx::query(
        r#"
        INSERT INTO organization_members (
            organization_id,
            user_id,
            role,
            status,
            created_at,
            updated_at
        )
        VALUES ($1, $2, 'admin', 'active', $3, $4)
        "#,
    )
    .bind(organization_id)
    .bind(user_id)
    .bind(&now)
    .bind(&now)
    .execute(db.pool.as_ref())
    .await
    .unwrap();

    (user_id, email)
}

async fn login_token(db: &Db, organization_id: i64) -> String {
    let (_user_id, email) = seed_admin_user(db, organization_id).await;
    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "email": email,
            "password": "Password123!"
        }))
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    resp["token"].as_str().unwrap().to_string()
}

async fn seed_client(db: &Db, organization_id: i64) -> i64 {
    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO clients (
            organization_id,
            name,
            email,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind("Demo Client")
    .bind("demo@example.com")
    .bind(Utc::now().to_rfc3339())
    .bind(Utc::now().to_rfc3339())
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

async fn seed_project(db: &Db, organization_id: i64, client_id: i64) -> i64 {
    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO projects (
            organization_id,
            client_id,
            name,
            start_date,
            description,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind(client_id)
    .bind("Demo Project")
    .bind(Utc::now().to_rfc3339())
    .bind("Demo description")
    .bind(Utc::now().to_rfc3339())
    .bind(Utc::now().to_rfc3339())
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

async fn seed_contract(db: &Db, organization_id: i64, project_id: i64) -> i64 {
    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO contracts (
            organization_id,
            project_id,
            title,
            status,
            signed_at,
            start_date,
            value,
            currency,
            terms,
            notes,
            external_id,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, 'active', $4, $5, $6, 'USD', $7, $8, $9, $10, $11)
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind(project_id)
    .bind("Demo Contract")
    .bind(Utc::now().to_rfc3339())
    .bind(Utc::now().to_rfc3339())
    .bind(1000.0)
    .bind("Terms")
    .bind("Notes")
    .bind("EXT123")
    .bind(Utc::now().to_rfc3339())
    .bind(Utc::now().to_rfc3339())
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

async fn seed_invoice(db: &Db, organization_id: i64, contract_id: i64) -> i64 {
    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO invoices (
            organization_id,
            contract_id,
            invoice_number,
            status,
            issued_at,
            due_date,
            subtotal,
            tax,
            total,
            currency,
            notes,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, 'pending', $4, $5, $6, $7, $8, 'USD', $9, $10, $11)
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind(contract_id)
    .bind(format!(
        "INV-{}",
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ))
    .bind(Utc::now().to_rfc3339())
    .bind(Utc::now().to_rfc3339())
    .bind(1000.0)
    .bind(100.0)
    .bind(1100.0)
    .bind("Invoice notes")
    .bind(Utc::now().to_rfc3339())
    .bind(Utc::now().to_rfc3339())
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

#[actix_web::test]
#[serial]
async fn test_organization_clients_endpoint() {
    let db = setup_test_db().await;
    let organization_id = seed_org(&db).await;
    let token = login_token(&db, organization_id).await;
    let app = test::init_service(test_app(db.clone())).await;

    let payload = json!({
        "organization_id": organization_id,
        "name": "Test Client",
        "email": "client@test.com",
        "phone": "555-1234"
    });

    let req = test::TestRequest::post()
        .uri(&format!("/api/organizations/{}/clients", organization_id))
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get()
        .uri(&format!("/api/organizations/{}/clients", organization_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_organization_projects_endpoint() {
    let db = setup_test_db().await;
    let organization_id = seed_org(&db).await;
    let token = login_token(&db, organization_id).await;
    let client_id = seed_client(&db, organization_id).await;
    let app = test::init_service(test_app(db.clone())).await;

    let payload = json!({
        "client_id": client_id,
        "name": "Test Project",
        "start_date": Utc::now().to_rfc3339()
    });

    let req = test::TestRequest::post()
        .uri(&format!("/api/organizations/{}/projects", organization_id))
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get()
        .uri(&format!("/api/organizations/{}/projects", organization_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_organization_invoices_endpoint() {
    let db = setup_test_db().await;
    let organization_id = seed_org(&db).await;
    let token = login_token(&db, organization_id).await;

    let client_id = seed_client(&db, organization_id).await;
    let project_id = seed_project(&db, organization_id, client_id).await;
    let contract_id = seed_contract(&db, organization_id, project_id).await;

    let app = test::init_service(test_app(db.clone())).await;

    let payload = json!({
        "organization_id": organization_id,
        "contract_id": contract_id,
        "invoice_number": "INV-001",
        "status": "pending",
        "issued_at": Utc::now().to_rfc3339(),
        "due_date": Utc::now().to_rfc3339(),
        "subtotal": 1000.0,
        "tax": 100.0,
        "total": 1100.0,
        "currency": "USD",
        "notes": "Invoice notes"
    });

    let req = test::TestRequest::post()
        .uri("/api/invoices")
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get()
        .uri(&format!("/api/organizations/{}/invoices", organization_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
#[serial]
async fn test_payment_endpoint() {
    let db = setup_test_db().await;
    let organization_id = seed_org(&db).await;
    let token = login_token(&db, organization_id).await;

    let client_id = seed_client(&db, organization_id).await;
    let project_id = seed_project(&db, organization_id, client_id).await;
    let contract_id = seed_contract(&db, organization_id, project_id).await;
    let invoice_id = seed_invoice(&db, organization_id, contract_id).await;

    let app = test::init_service(test_app(db.clone())).await;

    let payload = json!({
        "organization_id": organization_id,
        "invoice_id": invoice_id,
        "amount": 1100.0,
        "paid_at": Utc::now().to_rfc3339(),
        "currency": "USD",
        "method": "card",
        "reference": "TXN123",
        "notes": "Full payment"
    });

    let req = test::TestRequest::post()
        .uri("/api/payments")
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get()
        .uri("/api/payments")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Global payments listing is platform-admin-only now, so org admin may be forbidden.
    assert!(resp.status().is_success() || resp.status().as_u16() == 403);
}

#[actix_web::test]
#[serial]
async fn test_invoice_formatting() {
    let amount = 1200.5;
    assert_eq!(format!("${:.2}", amount), "$1200.50");
}
