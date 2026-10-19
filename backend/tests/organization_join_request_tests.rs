mod common;

use actix_web::{http::StatusCode, test};
use backend::db::Db;
use common::{setup_test_db, test_app};
use serde_json::{Value, json};
use serial_test::serial;

async fn seed_user(db: &Db, email: &str) -> i64 {
    let hash = backend::auth::hash_password("Password123!").unwrap();

    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO users (
            email, password_hash, name, user_type,
            email_verified_at, created_at, updated_at
        )
        VALUES ($1, $2, $1, 'owner',
                NOW()::TEXT, NOW()::TEXT, NOW()::TEXT)
        RETURNING id
        "#,
    )
    .bind(email)
    .bind(hash)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

async fn seed_org(db: &Db, visibility: &str) -> i64 {
    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO organizations (
            name, slug, visibility, created_at, updated_at
        )
        VALUES (
            'Onboarding Test',
            concat('onboarding-', gen_random_uuid()),
            $1, NOW()::TEXT, NOW()::TEXT
        )
        RETURNING id
        "#,
    )
    .bind(visibility)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

async fn seed_membership(db: &Db, org: i64, user: i64, role: &str, status: &str) {
    sqlx::query(
        r#"
        INSERT INTO organization_members (
            organization_id, user_id, role, status,
            created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, NOW()::TEXT, NOW()::TEXT)
        "#,
    )
    .bind(org)
    .bind(user)
    .bind(role)
    .bind(status)
    .execute(db.pool.as_ref())
    .await
    .unwrap();
}

async fn login(db: &Db, email: &str) -> String {
    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "email": email,
            "password": "Password123!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;

    assert!(
        status.is_success(),
        "Login failed: {} {}",
        status,
        String::from_utf8_lossy(&body)
    );

    let data: Value = serde_json::from_slice(&body).unwrap();
    data["token"].as_str().unwrap().to_string()
}

async fn post(db: &Db, token: &str, uri: &str) -> (StatusCode, Value) {
    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;

    let data = serde_json::from_slice(&body).unwrap_or(Value::Null);
    (status, data)
}

async fn get(db: &Db, token: &str, uri: &str) -> (StatusCode, Value) {
    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::get()
        .uri(uri)
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;

    let data = serde_json::from_slice(&body).unwrap_or(Value::Null);
    (status, data)
}

#[actix_web::test]
#[serial]
async fn discovery_excludes_private_organizations() {
    let db = setup_test_db().await;
    seed_org(&db, "public").await;
    seed_org(&db, "private").await;

    seed_user(&db, "discover@example.com").await;
    let token = login(&db, "discover@example.com").await;

    let (status, body) = get(&db, &token, "/api/organizations/discover").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 1);
}

#[actix_web::test]
#[serial]
async fn private_organization_rejects_join_request() {
    let db = setup_test_db().await;
    let org = seed_org(&db, "private").await;

    seed_user(&db, "private@example.com").await;
    let token = login(&db, "private@example.com").await;

    let (status, _) = post(
        &db,
        &token,
        &format!("/api/organizations/{org}/join-requests"),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[actix_web::test]
#[serial]
async fn duplicate_pending_request_returns_conflict() {
    let db = setup_test_db().await;
    let org = seed_org(&db, "public").await;

    seed_user(&db, "duplicate@example.com").await;
    let token = login(&db, "duplicate@example.com").await;

    let uri = format!("/api/organizations/{org}/join-requests");

    let (first, _) = post(&db, &token, &uri).await;
    let (second, _) = post(&db, &token, &uri).await;

    assert_eq!(first, StatusCode::CREATED);
    assert_eq!(second, StatusCode::CONFLICT);
}

#[actix_web::test]
#[serial]
async fn existing_member_cannot_request_to_join() {
    let db = setup_test_db().await;
    let org = seed_org(&db, "public").await;
    let user = seed_user(&db, "existing@example.com").await;

    seed_membership(&db, org, user, "member", "active").await;
    let token = login(&db, "existing@example.com").await;

    let (status, _) = post(
        &db,
        &token,
        &format!("/api/organizations/{org}/join-requests"),
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
}

#[actix_web::test]
#[serial]
async fn owner_can_approve_request_and_create_membership() {
    let db = setup_test_db().await;
    let org = seed_org(&db, "public").await;

    let owner = seed_user(&db, "owner@example.com").await;
    let applicant = seed_user(&db, "applicant@example.com").await;

    seed_membership(&db, org, owner, "owner", "active").await;

    let applicant_token = login(&db, "applicant@example.com").await;
    let owner_token = login(&db, "owner@example.com").await;

    let (created, request) = post(
        &db,
        &applicant_token,
        &format!("/api/organizations/{org}/join-requests"),
    )
    .await;

    assert_eq!(created, StatusCode::CREATED);

    let request_id = request["id"].as_i64().unwrap();

    let uri = format!("/api/organizations/{org}/join-requests/{request_id}/approve");

    let (approved, _) = post(&db, &owner_token, &uri).await;
    assert_eq!(approved, StatusCode::OK);

    let membership: (String, String) = sqlx::query_as(
        r#"
        SELECT role, status
        FROM organization_members
        WHERE organization_id = $1 AND user_id = $2
        "#,
    )
    .bind(org)
    .bind(applicant)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(membership.0, "member");
    assert_eq!(membership.1, "active");

    let (again, _) = post(&db, &owner_token, &uri).await;
    assert_eq!(again, StatusCode::CONFLICT);
}

#[actix_web::test]
#[serial]
async fn regular_member_cannot_approve_request() {
    let db = setup_test_db().await;
    let org = seed_org(&db, "public").await;

    let reviewer = seed_user(&db, "reviewer@example.com").await;
    let applicant = seed_user(&db, "blocked@example.com").await;

    seed_membership(&db, org, reviewer, "member", "active").await;

    let reviewer_token = login(&db, "reviewer@example.com").await;
    let applicant_token = login(&db, "blocked@example.com").await;

    let (_, request) = post(
        &db,
        &applicant_token,
        &format!("/api/organizations/{org}/join-requests"),
    )
    .await;

    let request_id = request["id"].as_i64().unwrap();

    let (status, _) = post(
        &db,
        &reviewer_token,
        &format!("/api/organizations/{org}/join-requests/{request_id}/approve"),
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN);

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM organization_members WHERE organization_id = $1 AND user_id = $2",
    )
    .bind(org)
    .bind(applicant)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(count, 0);
}

#[actix_web::test]
#[serial]
async fn rejection_does_not_create_membership() {
    let db = setup_test_db().await;
    let org = seed_org(&db, "public").await;

    let owner = seed_user(&db, "reject-owner@example.com").await;
    let applicant = seed_user(&db, "reject-applicant@example.com").await;

    seed_membership(&db, org, owner, "owner", "active").await;

    let owner_token = login(&db, "reject-owner@example.com").await;
    let applicant_token = login(&db, "reject-applicant@example.com").await;

    let (_, request) = post(
        &db,
        &applicant_token,
        &format!("/api/organizations/{org}/join-requests"),
    )
    .await;

    let request_id = request["id"].as_i64().unwrap();

    let (status, _) = post(
        &db,
        &owner_token,
        &format!("/api/organizations/{org}/join-requests/{request_id}/reject"),
    )
    .await;

    assert_eq!(status, StatusCode::OK);

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM organization_members WHERE organization_id = $1 AND user_id = $2",
    )
    .bind(org)
    .bind(applicant)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(count, 0);
}

#[actix_web::test]
#[serial]
async fn suspended_member_cannot_be_reactivated_by_approval() {
    let db = setup_test_db().await;
    let org = seed_org(&db, "public").await;

    let owner = seed_user(&db, "suspend-owner@example.com").await;
    let applicant = seed_user(&db, "suspend-applicant@example.com").await;

    seed_membership(&db, org, owner, "owner", "active").await;

    let applicant_token = login(&db, "suspend-applicant@example.com").await;
    let owner_token = login(&db, "suspend-owner@example.com").await;

    let (_, request) = post(
        &db,
        &applicant_token,
        &format!("/api/organizations/{org}/join-requests"),
    )
    .await;

    seed_membership(&db, org, applicant, "member", "suspended").await;

    let request_id = request["id"].as_i64().unwrap();

    let (status, _) = post(
        &db,
        &owner_token,
        &format!("/api/organizations/{org}/join-requests/{request_id}/approve"),
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);

    let membership_status: String = sqlx::query_scalar(
        "SELECT status FROM organization_members WHERE organization_id = $1 AND user_id = $2",
    )
    .bind(org)
    .bind(applicant)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(membership_status, "suspended");
}
