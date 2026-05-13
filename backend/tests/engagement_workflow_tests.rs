use actix_web::{App, test, web};
use backend::db::Db;
use backend::domain::engagement_state::EngagementEvent;
use backend::domain::engagement_state::EngagementStatus;
use backend::handlers::{engagement, engagement_milestone, software_contract};
use backend::services::operations_kernel_service::OperationsKernelService;
use serde_json::json;
use sqlx::{Executor, SqlitePool};

async fn setup_db() -> Db {
    use std::sync::Arc;

    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to create in-memory sqlite db");

    // organizations
    pool.execute(
        r#"
        CREATE TABLE organizations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .await
    .unwrap();

    // projects
    pool.execute(
        r#"
        CREATE TABLE projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            organization_id INTEGER NOT NULL,
            client_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            start_date TEXT,
            end_date TEXT,
            description TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .await
    .unwrap();

    // engagements
    pool.execute(
        r#"
        CREATE TABLE engagements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            organization_id INTEGER NOT NULL,
            project_id INTEGER NOT NULL,
            contractor_name TEXT NOT NULL,
            contractor_email TEXT NOT NULL,
            role TEXT NOT NULL,
            title TEXT NOT NULL,
            scope_of_work TEXT NOT NULL,
            deliverables TEXT,
            repo_url TEXT,
            amount_cents INTEGER NOT NULL,
            currency TEXT NOT NULL DEFAULT 'usd',
            due_date TEXT,
            status TEXT NOT NULL DEFAULT 'draft',
            platform_fee_status TEXT NOT NULL DEFAULT 'pending',
            contract_id INTEGER,
            invoice_id INTEGER,
            payment_id INTEGER,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .await
    .unwrap();

    // engagement milestones
    pool.execute(
        r#"
        CREATE TABLE engagement_milestones (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            engagement_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            amount_cents INTEGER NOT NULL DEFAULT 0,
            due_date TEXT,
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .await
    .unwrap();

    // engagement billing
    pool.execute(
        r#"
        CREATE TABLE engagement_billing (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            engagement_id INTEGER NOT NULL,
            organization_id INTEGER NOT NULL,
            billing_type TEXT NOT NULL,
            amount_cents INTEGER NOT NULL,
            currency TEXT NOT NULL DEFAULT 'usd',
            status TEXT NOT NULL DEFAULT 'pending',
            stripe_checkout_session_id TEXT,
            stripe_payment_intent_id TEXT,
            paid_at TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .await
    .unwrap();

    // seed org
    pool.execute(
        r#"
        INSERT INTO organizations (name)
        VALUES ('Test Org');
        "#,
    )
    .await
    .unwrap();

    // seed project
    pool.execute(
        r#"
        INSERT INTO projects (
            organization_id,
            client_id,
            name,
            description
        )
        VALUES (
            1,
            1,
            'Client Portal MVP',
            'Build a working software client portal'
        );
        "#,
    )
    .await
    .unwrap();

    Db {
        pool: Arc::new(pool),
    }
}
fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route(
                "/projects/{project_id}/engagements",
                web::post().to(engagement::create_for_project),
            )
            .route(
                "/projects/{project_id}/engagements",
                web::get().to(engagement::list_for_project),
            )
            .route("/engagements/{id}", web::get().to(engagement::show))
            .route(
                "/engagements/{id}/mark-contract-sent",
                web::post().to(engagement::mark_contract_sent),
            )
            .route(
                "/engagements/{id}/mark-signed",
                web::post().to(engagement::mark_signed),
            )
            .route(
                "/engagements/{id}/milestones",
                web::post().to(engagement_milestone::create_engagement_milestone),
            )
            .route(
                "/engagements/{id}/milestones",
                web::get().to(engagement_milestone::list_engagement_milestones),
            )
            .route(
                "/milestones/{id}/submit",
                web::post().to(engagement_milestone::submit_engagement_milestone),
            )
            .route(
                "/milestones/{id}/approve",
                web::post().to(engagement_milestone::approve_engagement_milestone),
            )
            .route(
                "/milestones/{id}/mark-paid",
                web::post().to(engagement_milestone::mark_engagement_milestone_paid),
            )
            .route(
                "/engagements/{id}/software-contract",
                web::post().to(software_contract::generate_for_engagement),
            ),
    );
}

#[actix_rt::test]
async fn creates_software_engagement_for_project() {
    let db = setup_db().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(app_config),
    )
    .await;

    let payload = json!({
        "contractor_name": "Peter Dev",
        "contractor_email": "peter@example.com",
        "role": "full_stack_developer",
        "title": "Build Client Portal MVP",
        "scope_of_work": "Build auth, project dashboard, milestones, and payment workflow.",
        "deliverables": "Rust API, Nuxt frontend, working deployment",
        "repo_url": "https://github.com/example/client-portal",
        "amount_cents": 200000,
        "currency": "usd",
        "due_date": "2026-06-01"
    });

    let req = test::TestRequest::post()
        .uri("/api/projects/1/engagements")
        .set_json(&payload)
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp["organization_id"], 1);
    assert_eq!(resp["project_id"], 1);
    assert_eq!(resp["contractor_name"], "Peter Dev");
    assert_eq!(resp["status"], "draft");
    assert_eq!(resp["platform_fee_status"], "pending");
}

#[actix_rt::test]
async fn lists_engagements_for_project() {
    let db = setup_db().await;

    sqlx::query(
        r#" 
        INSERT INTO engagements (
            organization_id,
            project_id,
            contractor_name,
            contractor_email,
            role,
            title,
            scope_of_work,
            amount_cents,
            currency,
            status,
            platform_fee_status
        )
        VALUES (
            1,
            1,
            'Contractor One',
            'contractor@example.com',
            'backend_developer',
            'API Build',
            'Build backend API',
            100000,
            'usd',
            'draft',
            'pending'
        );
        "#,
    )
    .execute(db.pool.as_ref())
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(app_config),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/projects/1/engagements")
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert!(resp.as_array().unwrap().len() >= 1);
    assert_eq!(resp[0]["project_id"], 1);
}

#[actix_rt::test]
async fn creates_and_lists_milestones() {
    let db = setup_db().await;

    sqlx::query(
        r#"
        INSERT INTO engagements (
            organization_id,
            project_id,
            contractor_name,
            contractor_email,
            role,
            title,
            scope_of_work,
            amount_cents,
            currency,
            status,
            platform_fee_status
        )
        VALUES (
            1,
            1,
            'Milestone Dev',
            'milestone@example.com',
            'frontend_developer',
            'Frontend Build',
            'Build Nuxt workflow screens',
            150000,
            'usd',
            'draft',
            'pending'
        );
        "#,
    )
    .execute(db.pool.as_ref())
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(app_config),
    )
    .await;

    let payload = json!({
        "title": "Build Engagement Wizard",
        "description": "Create the new engagement form and review screen",
        "amount_cents": 50000,
        "due_date": "2026-05-15"
    });

    let create_req = test::TestRequest::post()
        .uri("/api/engagements/1/milestones")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, create_req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    let body_text = String::from_utf8_lossy(&body);

    assert!(
        status.is_success(),
        "Expected milestone create success but got {}: {}",
        status,
        body_text
    );

    let created: serde_json::Value =
        serde_json::from_slice(&body).expect("milestone create response was not JSON");
    assert_eq!(created["engagement_id"], 1);
    assert_eq!(created["title"], "Build Engagement Wizard");
    assert_eq!(created["status"], "pending");

    let list_req = test::TestRequest::get()
        .uri("/api/engagements/1/milestones")
        .to_request();

    let list: serde_json::Value = test::call_and_read_body_json(&app, list_req).await;

    assert_eq!(list.as_array().unwrap().len(), 1);
}

#[actix_rt::test]
async fn updates_milestone_status_flow() {
    let db = setup_db().await;

    sqlx::query(
        r#"
        INSERT INTO engagement_milestones (
            engagement_id,
            title,
            description,
            amount_cents,
            status
        )
        VALUES (
            1,
            'Auth Flow',
            'Build login and org switcher',
            40000,
            'pending'
        );
        "#,
    )
    .execute(db.pool.as_ref())
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(app_config),
    )
    .await;

    let submit_req = test::TestRequest::post()
        .uri("/api/milestones/1/submit")
        .to_request();

    let submitted: serde_json::Value = test::call_and_read_body_json(&app, submit_req).await;
    assert_eq!(submitted["status"], "submitted");

    let approve_req = test::TestRequest::post()
        .uri("/api/milestones/1/approve")
        .to_request();

    let approved: serde_json::Value = test::call_and_read_body_json(&app, approve_req).await;
    assert_eq!(approved["status"], "approved");

    let paid_req = test::TestRequest::post()
        .uri("/api/milestones/1/mark-paid")
        .to_request();

    let paid: serde_json::Value = test::call_and_read_body_json(&app, paid_req).await;
    assert_eq!(paid["status"], "paid");
}

#[actix_rt::test]
async fn generates_software_contract_body() {
    let db = setup_db().await;

    sqlx::query(
        r#"
        INSERT INTO engagements (
            organization_id,
            project_id,
            contractor_name,
            contractor_email,
            role,
            title,
            scope_of_work,
            deliverables,
            repo_url,
            amount_cents,
            currency,
            status,
            platform_fee_status
        )
        VALUES (
            1,
            1,
            'Software Contractor',
            'dev@example.com',
            'full_stack_developer',
            'Build SaaS MVP',
            'Build Rust API and Nuxt frontend',
            'Auth, billing, dashboard, tracker',
            'https://github.com/example/saas',
            200000,
            'usd',
            'draft',
            'pending'
        );
        "#,
    )
    .execute(db.pool.as_ref())
    .await
    .unwrap();

    sqlx::query(
        r#"
        INSERT INTO engagement_milestones (
            engagement_id,
            title,
            description,
            amount_cents,
            status
        )
        VALUES (
            1,
            'Stripe Billing',
            'Implement activation fee checkout',
            50000,
            'pending'
        );
        "#,
    )
    .execute(db.pool.as_ref())
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(app_config),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/engagements/1/software-contract")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    let body_text = String::from_utf8_lossy(&body);

    assert!(
        status.is_success(),
        "Expected software contract success but got {}: {}",
        status,
        body_text
    );

    let resp: serde_json::Value =
        serde_json::from_slice(&body).expect("software contract response was not JSON");
    assert_eq!(resp["contract_type"], "software_services");
}

#[actix_rt::test]
async fn payment_received_from_draft_should_fail() {
    let db = setup_db().await;

    let result = OperationsKernelService::apply_engagement_event(
        &db.pool.as_ref(),
        1,
        999,
        None,
        EngagementStatus::Draft,
        EngagementEvent::PaymentReceived,
    )
    .await;

    assert!(result.is_err());

    let error = result.err().unwrap();

    assert!(
        error.contains("Invalid engagement transition"),
        "Expected invalid transition error, got: {}",
        error
    );
}
