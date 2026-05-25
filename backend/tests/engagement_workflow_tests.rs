mod common;

use actix_web::test;
use backend::auth::hash_password;
use backend::db::Db;
use backend::domain::engagement_state::{EngagementEvent, EngagementStatus};
use backend::services::operations_kernel_service::OperationsKernelService;
use chrono::Utc;
use common::{setup_test_db, test_app};
use serde_json::json;
use serial_test::serial;

async fn seed_org_user_project(db: &Db) -> (i64, i64, String) {
    let now = Utc::now().to_rfc3339();

    let organization_id = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO organizations (name, slug, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind("Test Org")
    .bind(format!(
        "test-org-{}",
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ))
    .bind(&now)
    .bind(&now)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    let client_id = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO clients (organization_id, name, email, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind("Test Client")
    .bind("client@example.com")
    .bind(&now)
    .bind(&now)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    let project_id = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO projects (organization_id, client_id, name, description, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind(client_id)
    .bind("Client Portal MVP")
    .bind("Build a working software client portal")
    .bind(&now)
    .bind(&now)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    let email = format!(
        "owner-{}@example.com",
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    );
    let password_hash = hash_password("Password123!").unwrap();

    let user_id = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO users (
            email, password_hash, name, user_type, email_verified_at,
            current_organization_id, created_at, updated_at
        )
        VALUES ($1, $2, $3, 'admin', $4, $5, $6, $7)
        RETURNING id
        "#,
    )
    .bind(&email)
    .bind(password_hash)
    .bind("Test Owner")
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
            organization_id, user_id, role, status, created_at, updated_at
        )
        VALUES ($1, $2, 'owner', 'active', $3, $4)
        "#,
    )
    .bind(organization_id)
    .bind(user_id)
    .bind(&now)
    .bind(&now)
    .execute(db.pool.as_ref())
    .await
    .unwrap();

    (organization_id, project_id, email)
}

macro_rules! login_and_get_token {
    ($app:expr, $email:expr) => {{
        let req = test::TestRequest::post()
            .uri("/api/auth/login")
            .set_json(json!({
                "email": $email,
                "password": "Password123!"
            }))
            .to_request();

        let resp: serde_json::Value = test::call_and_read_body_json(&$app, req).await;
        resp["token"].as_str().unwrap().to_string()
    }};
}

async fn seed_engagement(db: &Db, organization_id: i64, project_id: i64) -> i64 {
    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO engagements (
            organization_id, project_id, engagement_type, contractor_name,
            contractor_email, role, title, scope_of_work, deliverables,
            repo_url, amount_cents, currency, status, platform_fee_status,
            created_at, updated_at
        )
        VALUES (
            $1, $2, 'software', 'Software Contractor', 'dev@example.com',
            'full_stack_developer', 'Build SaaS MVP',
            'Build Rust API and Nuxt frontend', 'Auth, billing, dashboard',
            'https://github.com/example/saas', 200000, 'usd',
            'draft', 'pending', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
        )
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind(project_id)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

#[actix_rt::test]
#[serial]
async fn creates_software_engagement_for_project() {
    let db = setup_test_db().await;
    let (_organization_id, project_id, email) = seed_org_user_project(&db).await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app, email);

    let payload = json!({
        "contractor_name": "Peter Dev",
        "contractor_email": "peter@example.com",
        "engagement_type": "software",
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
        .uri(&format!("/api/projects/{}/engagements", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&payload)
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp["project_id"], project_id);
    assert_eq!(resp["contractor_name"], "Peter Dev");
    assert_eq!(resp["status"], "draft");
    assert_eq!(resp["platform_fee_status"], "pending");
}

#[actix_rt::test]
#[serial]

async fn lists_engagements_for_project() {
    let db = setup_test_db().await;
    let (organization_id, project_id, email) = seed_org_user_project(&db).await;
    seed_engagement(&db, organization_id, project_id).await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app, email);

    let req = test::TestRequest::get()
        .uri(&format!("/api/projects/{}/engagements", project_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert!(resp.as_array().unwrap().len() >= 1);
    assert_eq!(resp[0]["project_id"], project_id);
}

#[actix_rt::test]
#[serial]

async fn creates_and_lists_milestones() {
    let db = setup_test_db().await;
    let (organization_id, project_id, email) = seed_org_user_project(&db).await;
    let engagement_id = seed_engagement(&db, organization_id, project_id).await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app, email);

    let payload = json!({
        "title": "Build Engagement Wizard",
        "description": "Create the new engagement form and review screen",
        "amount_cents": 50000,
        "due_date": "2026-05-15"
    });

    let create_req = test::TestRequest::post()
        .uri(&format!("/api/engagements/{}/milestones", engagement_id))
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .set_json(&payload)
        .to_request();

    let created: serde_json::Value = test::call_and_read_body_json(&app, create_req).await;
    assert_eq!(created["engagement_id"], engagement_id);
    assert_eq!(created["title"], "Build Engagement Wizard");
    assert_eq!(created["status"], "pending");

    let list_req = test::TestRequest::get()
        .uri(&format!("/api/engagements/{}/milestones", engagement_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let list: serde_json::Value = test::call_and_read_body_json(&app, list_req).await;
    assert_eq!(list.as_array().unwrap().len(), 1);
}

#[actix_rt::test]
#[serial]

async fn payment_received_from_draft_should_fail() {
    let db = setup_test_db().await;

    let result = OperationsKernelService::apply_engagement_event(
        db.pool.as_ref(),
        1,
        999,
        None,
        EngagementStatus::Draft,
        EngagementEvent::PaymentReceived,
    )
    .await;

    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .contains("Invalid engagement transition")
    );
}
