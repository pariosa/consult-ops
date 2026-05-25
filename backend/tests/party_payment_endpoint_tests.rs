mod common;

use actix_web::test;
use backend::auth::hash_password;
use chrono::Utc;
use common::{setup_test_db, test_app};

use serial_test::serial;
async fn seed_test_organization(db: &backend::db::Db) -> i64 {
    let now = Utc::now().to_rfc3339();
    let slug = format!(
        "atlas-test-org-{}",
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    );

    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO organizations (name, slug, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind("Atlas Test Org")
    .bind(slug)
    .bind(&now)
    .bind(&now)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

async fn seed_verified_admin(db: &backend::db::Db, organization_id: i64) -> i64 {
    let now = Utc::now().to_rfc3339();
    let password_hash = hash_password("DemoPass123!").unwrap();

    let user_id = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO users (
            email, password_hash, name, user_type,
            email_verified_at, current_organization_id, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
    )
    .bind("admin@atlas.test")
    .bind(password_hash)
    .bind("Atlas Admin")
    .bind("admin")
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

    user_id
}
#[actix_web::test]
#[serial]
async fn admin_can_create_party_and_read_default_payment_readiness() {
    let db = setup_test_db().await;
    let organization_id = seed_test_organization(&db).await;
    let _admin_id = seed_verified_admin(&db, organization_id).await;
    let app = test::init_service(test_app(db.clone())).await;

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "admin@atlas.test",
            "password": "DemoPass123!"
        }))
        .to_request();

    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;
    let token = login_resp["token"].as_str().unwrap().to_string();

    let req = test::TestRequest::post()
        .uri(&format!("/api/organizations/{}/parties", organization_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "Manual Contractor",
            "email": "contractor@example.com",
            "party_type": "contractor"
        }))
        .to_request();

    let created: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    println!(
        "CREATE PARTY BODY: {}",
        serde_json::to_string_pretty(&created).unwrap()
    );
    assert_eq!(created["name"], "Manual Contractor");
    assert_eq!(created["is_verified"], 0);
    assert_eq!(created["verification_status"], "unverified");

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/parties/{}/payment-readiness",
            organization_id,
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let readiness: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(readiness["is_verified"], false);
    assert_eq!(readiness["payer_ready"], false);
    assert_eq!(readiness["payee_ready"], false);
}

#[actix_web::test]
#[serial]
async fn admin_can_verify_party_and_authorize_payer_profile() {
    let db = setup_test_db().await;
    let organization_id = seed_test_organization(&db).await;
    let _admin_id = seed_verified_admin(&db, organization_id).await;
    sqlx::query(
        r#"
        INSERT INTO parties (
            organization_id,
            name,
            email,
            party_type,
            is_verified,
            verification_status
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(1)
    .bind("Riverbend Client")
    .bind("ops@riverbend.gov")
    .bind("client")
    .bind(0)
    .bind("unverified")
    .execute(&*db.pool)
    .await
    .unwrap();

    let app = test::init_service(test_app(db.clone())).await;

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "admin@atlas.test",
            "password": "DemoPass123!"
        }))
        .to_request();

    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;
    let token = login_resp["token"].as_str().unwrap().to_string();

    let req = test::TestRequest::post()
        .uri(&format!("/api/parties/{}/verify", organization_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    let body_text = String::from_utf8_lossy(&body);

    assert!(
        status.is_success(),
        "verify party failed with {} body: {}",
        status,
        body_text
    );

    let verified: serde_json::Value =
        serde_json::from_slice(&body).expect("verify party response should be JSON");
    assert_eq!(verified["is_verified"], 1);
    assert_eq!(verified["verification_status"], "verified");
    assert_eq!(verified["verification_method"], "admin");

    let req = test::TestRequest::post()
        .uri(&format!("/api/parties/{}/payment-profile", organization_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "payment_role": "payer",
            "payer_authorization_scope": "agreement"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    let body_text = String::from_utf8_lossy(&body);

    assert!(
        status.is_success(),
        "create payment profile failed with {} body: {}",
        status,
        body_text
    );

    let profile: serde_json::Value =
        serde_json::from_slice(&body).expect("payment profile response should be JSON");
    assert_eq!(profile["payment_role"], "payer");
    assert_eq!(profile["payer_authorization_status"], "not_configured");

    let req = test::TestRequest::post()
        .uri(&format!(
            "/api/parties/{}/payer-authorized/dev",
            organization_id,
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let authorized: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(authorized["payer_authorization_status"], "authorized");
    assert_eq!(authorized["stripe_customer_id"], "cus_dev_party_1");
    assert_eq!(authorized["stripe_payment_method_id"], "pm_dev_party_1");

    let req = test::TestRequest::get()
        .uri("/api/parties/1/payment-readiness")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let readiness: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(readiness["is_verified"], true);
    assert_eq!(readiness["payer_ready"], true);
    assert_eq!(readiness["payee_ready"], false);
}

#[actix_web::test]
#[serial]
async fn admin_can_mark_payee_payout_ready() {
    let db = setup_test_db().await;
    let organization_id = seed_test_organization(&db).await;
    let _admin_id = seed_verified_admin(&db, organization_id).await;
    let _user_id = sqlx::query(
        r#"
        INSERT INTO parties (
            organization_id,
            name,
            email,
            party_type,
            is_verified,
            verification_status,
            verified_at,
            verification_method
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW()::TEXT, $7)
        "#,
    )
    .bind(1)
    .bind("Riley Operations")
    .bind("ops@atlas.test")
    .bind("contractor")
    .bind(1)
    .bind("verified")
    .bind("admin")
    .execute(&*db.pool)
    .await
    .unwrap();

    let app = test::init_service(test_app(db.clone())).await;

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "admin@atlas.test",
            "password": "DemoPass123!"
        }))
        .to_request();

    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;
    let token = login_resp["token"].as_str().unwrap().to_string();

    let req = test::TestRequest::post()
        .uri(&format!("/api/parties/{}/payment-profile", organization_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "payment_role": "payee"
        }))
        .to_request();

    let profile: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(profile["payment_role"], "payee");
    assert_eq!(profile["payout_status"], "not_ready");

    let req = test::TestRequest::post()
        .uri("/api/parties/1/payout-ready/dev")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let ready: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(ready["stripe_connect_account_id"], "acct_dev_party_1");
    assert_eq!(ready["stripe_connect_onboarding_status"], "complete");
    assert_eq!(ready["payout_status"], "ready");

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/parties/{}/payment-readiness",
            organization_id,
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let readiness: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(readiness["is_verified"], true);
    assert_eq!(readiness["payer_ready"], false);
    assert_eq!(readiness["payee_ready"], true);
}
