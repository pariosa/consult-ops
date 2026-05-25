mod common;

use actix_web::{Responder, test};
use backend::auth::{AuthSessionResponse, hash_password, hash_token};
use chrono::{Duration, Utc};
use common::{setup_test_db, test_app};
use serial_test::serial;
async fn seed_user(
    db: &backend::db::Db,
    email: &str,
    password: &str,
    user_type: &str,
    verified: bool,
) -> i64 {
    let now = Utc::now().to_rfc3339();
    let password_hash = hash_password(password).unwrap();
    let verified_at: Option<String> = if verified { Some(now.clone()) } else { None };

    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO users (
            email,
            password_hash,
            name,
            user_type,
            email_verified_at,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(email)
    .bind(user_type)
    .bind(verified_at)
    .bind(&now)
    .bind(&now)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

async fn seed_verified_user(
    db: &backend::db::Db,
    email: Option<&str>,
    password: &str,
    user_type: &str,
) -> (i64, String) {
    let now = chrono::Utc::now().to_rfc3339();

    let email = email.map(|s| s.to_string()).unwrap_or_else(|| {
        format!(
            "verified-{}@example.com",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
        )
    });

    let password_hash = backend::auth::hash_password(password).unwrap();

    let user_id = sqlx::query_scalar::<_, i64>(
        r#"
    INSERT INTO users (
        email,
        password_hash,
        name,
        user_type,
        email_verified_at,
        created_at,
        updated_at
    )
    VALUES ($1, $2, $3, $4, NOW(), NOW(), NOW())
    RETURNING id
    "#,
    )
    .bind(&email)
    .bind(password_hash)
    .bind(&email)
    .bind(user_type)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    (user_id, email)
}
async fn seed_email_verification_token(db: &backend::db::Db, user_id: i64, raw_token: &str) {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO email_verification_tokens (
            user_id,
            token_hash,
            expires_at,
            created_at
        )
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(user_id)
    .bind(hash_token(raw_token))
    .bind((Utc::now() + Duration::minutes(30)).to_rfc3339())
    .bind(&now)
    .execute(db.pool.as_ref())
    .await
    .unwrap();
}

async fn seed_password_reset_token(db: &backend::db::Db, user_id: i64, raw_token: &str) {
    sqlx::query(
        r#"
        INSERT INTO password_reset_tokens (
            user_id,
            token_hash,
            expires_at,
            created_at
        )
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(user_id)
    .bind(hash_token(raw_token))
    .bind((Utc::now() + Duration::minutes(30)).to_rfc3339())
    .bind(Utc::now().to_rfc3339())
    .execute(db.pool.as_ref())
    .await
    .unwrap();
}

macro_rules! login_and_get_token {
    ($app:expr, $email:expr) => {{
        let req = test::TestRequest::post()
            .uri("/api/auth/login")
            .set_json(serde_json::json!({
                "email": $email,
                "password": "Password123!"
            }))
            .to_request();

        let resp = test::call_service(&$app, req).await;
        let status = resp.status();
        let body = test::read_body(resp).await;

        assert!(
            status.is_success(),
            "login failed: status={} body={}",
            status,
            String::from_utf8_lossy(&body)
        );

        let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();

        parsed["token"].as_str().unwrap().to_string()
    }};
}

#[actix_web::test]
#[serial]
async fn register_creates_unverified_user_and_email_verification_token() {
    let db = setup_test_db().await;
    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": "test@example.com",
            "password": "Password123!",
            "name": "Test User",
            "user_type": "owner"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let user: (Option<String>,) =
        sqlx::query_as("SELECT email_verified_at FROM users WHERE email = $1")
            .bind("test@example.com")
            .fetch_one(db.pool.as_ref())
            .await
            .unwrap();

    assert!(user.0.is_none());

    let token_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*)::BIGINT FROM email_verification_tokens")
            .fetch_one(db.pool.as_ref())
            .await
            .unwrap();

    assert_eq!(token_count.0, 1);
}

#[actix_web::test]
#[serial]
async fn login_rejects_unverified_user() {
    let db = setup_test_db().await;
    let app = test::init_service(test_app(db.clone())).await;

    seed_user(
        &db,
        "unverified@example.com",
        "Password123!",
        "owner",
        false,
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "unverified@example.com",
            "password": "Password123!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());

    let session_count: (i64,) = sqlx::query_as("SELECT COUNT(*)::BIGINT FROM auth_sessions")
        .fetch_one(db.pool.as_ref())
        .await
        .unwrap();

    assert_eq!(session_count.0, 0);
}

#[actix_web::test]
#[serial]
async fn verify_email_accepts_valid_token_once() {
    let db = setup_test_db().await;
    let app = test::init_service(test_app(db.clone())).await;

    let user_id = seed_user(&db, "verify@example.com", "Password123!", "owner", false).await;
    let raw_token = "valid-email-verification-token";

    seed_email_verification_token(&db, user_id, raw_token).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/verify-email")
        .set_json(serde_json::json!({ "token": raw_token }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let verified_at: (Option<String>,) =
        sqlx::query_as("SELECT email_verified_at FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(db.pool.as_ref())
            .await
            .unwrap();

    assert!(verified_at.0.is_some());

    let used_at: (Option<String>,) =
        sqlx::query_as("SELECT used_at FROM email_verification_tokens WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(db.pool.as_ref())
            .await
            .unwrap();

    assert!(used_at.0.is_some());

    let second_req = test::TestRequest::post()
        .uri("/api/auth/verify-email")
        .set_json(serde_json::json!({ "token": raw_token }))
        .to_request();

    let second_resp = test::call_service(&app, second_req).await;
    assert!(second_resp.status().is_client_error());
}

#[actix_web::test]
#[serial]
async fn forgot_password_never_reveals_whether_email_exists() {
    let db = setup_test_db().await;
    let app = test::init_service(test_app(db.clone())).await;

    seed_verified_user(&db, Some("existing@example.com"), "Password123!", "owner").await;

    let existing_req = test::TestRequest::post()
        .uri("/api/auth/forgot-password")
        .set_json(serde_json::json!({ "email": "existing@example.com" }))
        .to_request();

    let existing_resp = test::call_service(&app, existing_req).await;
    let existing_status = existing_resp.status();
    let existing_body = test::read_body(existing_resp).await;

    let missing_req = test::TestRequest::post()
        .uri("/api/auth/forgot-password")
        .set_json(serde_json::json!({ "email": "missing@example.com" }))
        .to_request();

    let missing_resp = test::call_service(&app, missing_req).await;
    let missing_status = missing_resp.status();
    let missing_body = test::read_body(missing_resp).await;

    assert_eq!(existing_status, missing_status);
    assert_eq!(existing_body, missing_body);
}

#[actix_web::test]
#[serial]
async fn login_creates_active_auth_session() {
    let db = setup_test_db().await;
    let (_user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": email,
            "password": "Password123!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;

    assert!(
        status.is_success(),
        "login failed: status={} body={}",
        status,
        String::from_utf8_lossy(&body)
    );

    let resp: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(resp["token"].as_str().is_some());

    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*)::BIGINT FROM auth_sessions WHERE revoked_at IS NULL")
            .fetch_one(db.pool.as_ref())
            .await
            .unwrap();

    assert_eq!(count.0, 1);
}

#[actix_web::test]
#[serial]
async fn logout_revokes_current_session() {
    let db = setup_test_db().await;
    let (_user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;
    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app, &email);

    let req = test::TestRequest::post()
        .uri("/api/auth/logout")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let revoked_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*)::BIGINT FROM auth_sessions WHERE revoked_at IS NOT NULL")
            .fetch_one(db.pool.as_ref())
            .await
            .unwrap();

    assert_eq!(revoked_count.0, 1);
}

#[actix_web::test]
#[serial]
async fn revoked_session_cannot_access_me() {
    let db = setup_test_db().await;
    let (_user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;
    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app, &email);

    let logout_req = test::TestRequest::post()
        .uri("/api/auth/logout")
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .to_request();

    assert_eq!(
        test::call_service(&app, logout_req).await.status().as_u16(),
        200
    );

    let me_req = test::TestRequest::get()
        .uri("/api/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    assert_eq!(
        test::call_service(&app, me_req).await.status().as_u16(),
        401
    );
}

#[actix_web::test]
#[serial]
async fn expired_session_cannot_access_me() {
    let db = setup_test_db().await;
    let (user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;
    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app, &email);

    sqlx::query(
        r#"
        UPDATE auth_sessions
        SET expires_at = $1
        WHERE user_id = $2
        "#,
    )
    .bind((Utc::now() - Duration::hours(1)).to_rfc3339())
    .bind(user_id)
    .execute(db.pool.as_ref())
    .await
    .unwrap();

    let req = test::TestRequest::get()
        .uri("/api/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    assert_eq!(test::call_service(&app, req).await.status().as_u16(), 401);
}

#[actix_web::test]
#[serial]
async fn password_reset_invalidates_existing_sessions() {
    let db = setup_test_db().await;
    let (user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;
    let raw_reset_token = "valid-reset-token";
    let token = login_and_get_token!(app, &email);
    seed_password_reset_token(&db, user_id, raw_reset_token).await;

    let reset_req = test::TestRequest::post()
        .uri("/api/auth/reset-password")
        .set_json(serde_json::json!({
            "token": raw_reset_token,
            "password": "NewPassword123!"
        }))
        .to_request();

    assert_eq!(
        test::call_service(&app, reset_req).await.status().as_u16(),
        200
    );

    let me_req = test::TestRequest::get()
        .uri("/api/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    assert_eq!(
        test::call_service(&app, me_req).await.status().as_u16(),
        401
    );
}

#[actix_web::test]
#[serial]
async fn repeated_failed_logins_are_throttled() {
    let db = setup_test_db().await;
    let (_user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;
    let app = test::init_service(test_app(db.clone())).await;

    for _ in 0..5 {
        let req = test::TestRequest::post()
            .uri("/api/auth/login")
            .set_json(serde_json::json!({
                "email": email,
                "password": "WrongPassword!"
            }))
            .to_request();

        assert!(
            test::call_service(&app, req)
                .await
                .status()
                .is_client_error()
        );
    }

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": email,
            "password": "WrongPassword!"
        }))
        .to_request();

    assert_eq!(test::call_service(&app, req).await.status().as_u16(), 429);
}

#[actix_web::test]
#[serial]
async fn successful_login_creates_audit_event() {
    let db = setup_test_db().await;
    let (_user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;
    let app = test::init_service(test_app(db.clone())).await;
    let _token = login_and_get_token!(app, &email);

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::BIGINT FROM audit_events WHERE event_type = 'auth.login_success'",
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(count.0, 1);
}

#[actix_web::test]
#[serial]

async fn failed_login_creates_audit_event() {
    let db = setup_test_db().await;
    let (_user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;
    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": email,
            "password": "WrongPassword!"
        }))
        .to_request();

    assert_eq!(test::call_service(&app, req).await.status().as_u16(), 401);

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::BIGINT FROM audit_events WHERE event_type = 'auth.login_failed'",
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(count.0, 1);
}

#[actix_web::test]
#[serial]
async fn password_reset_creates_audit_event() {
    let db = setup_test_db().await;
    let (user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;
    let app = test::init_service(test_app(db.clone())).await;
    let raw_reset_token = "valid-reset-token";

    seed_password_reset_token(&db, user_id, raw_reset_token).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/reset-password")
        .set_json(serde_json::json!({
            "token": raw_reset_token,
            "password": "NewPassword123!"
        }))
        .to_request();

    assert_eq!(test::call_service(&app, req).await.status().as_u16(), 200);

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::BIGINT FROM audit_events WHERE event_type = 'auth.password_reset_success'",
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(count.0, 1);
}

#[actix_web::test]
#[serial]
async fn email_verification_creates_audit_event() {
    let db = setup_test_db().await;
    let user_id = seed_user(
        &db,
        "verify-audit@example.com",
        "Password123!",
        "owner",
        false,
    )
    .await;
    let raw_token = "valid-email-token";

    seed_email_verification_token(&db, user_id, raw_token).await;

    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/verify-email")
        .set_json(serde_json::json!({ "token": raw_token }))
        .to_request();

    assert_eq!(test::call_service(&app, req).await.status().as_u16(), 200);

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::BIGINT FROM audit_events WHERE event_type = 'auth.email_verified'",
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(count.0, 1);
}

#[actix_web::test]
#[serial]
async fn auth_me_returns_current_verified_user() {
    let db = setup_test_db().await;
    let (_user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;
    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app, &email);

    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp["email"], email);
    assert_eq!(resp["user_type"], "owner");
    assert!(resp["email_verified_at"].as_str().is_some());
    assert!(resp["active_session_id"].as_i64().is_some());
}

#[actix_web::test]
#[serial]
async fn resend_verification_creates_new_token_for_unverified_user() {
    let db = setup_test_db().await;
    let user_id = seed_user(
        &db,
        "unverified-resend@example.com",
        "Password123!",
        "owner",
        false,
    )
    .await;

    seed_email_verification_token(&db, user_id, "old-token").await;

    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/resend-verification")
        .set_json(serde_json::json!({
            "email": "unverified-resend@example.com"
        }))
        .to_request();

    assert!(test::call_service(&app, req).await.status().is_success());

    let unused_count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM email_verification_tokens
        WHERE user_id = $1
          AND used_at IS NULL
        "#,
    )
    .bind(user_id)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(unused_count.0, 1);

    let total_count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM email_verification_tokens
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(total_count.0, 2);
}

#[actix_web::test]
#[serial]
async fn resend_verification_does_not_reveal_missing_email() {
    let db = setup_test_db().await;
    seed_user(
        &db,
        "real-unverified@example.com",
        "Password123!",
        "owner",
        false,
    )
    .await;

    let app = test::init_service(test_app(db.clone())).await;

    let existing_req = test::TestRequest::post()
        .uri("/api/auth/resend-verification")
        .set_json(serde_json::json!({
            "email": "real-unverified@example.com"
        }))
        .to_request();

    let existing_resp = test::call_service(&app, existing_req).await;
    let existing_status = existing_resp.status();
    let existing_body = test::read_body(existing_resp).await;

    let missing_req = test::TestRequest::post()
        .uri("/api/auth/resend-verification")
        .set_json(serde_json::json!({
            "email": "missing@example.com"
        }))
        .to_request();

    let missing_resp = test::call_service(&app, missing_req).await;
    let missing_status = missing_resp.status();
    let missing_body = test::read_body(missing_resp).await;

    assert_eq!(existing_status, missing_status);
    assert_eq!(existing_body, missing_body);
}

#[actix_web::test]
#[serial]
async fn user_can_list_auth_sessions() {
    let db = setup_test_db().await;
    let (_user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app, &email);

    let req = test::TestRequest::get()
        .uri("/api/auth/sessions")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;

    assert!(
        status.is_success(),
        "remember_me login failed: status={} body={}",
        status,
        String::from_utf8_lossy(&body)
    );

    let sessions: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(sessions.as_array().unwrap().len(), 1);
    assert!(sessions[0]["id"].as_i64().is_some());
    assert!(sessions[0]["expires_at"].as_str().is_some());
    assert!(sessions[0]["revoked_at"].is_null());
}
#[actix_web::test]
#[serial]
async fn user_can_revoke_auth_session() {
    let db = setup_test_db().await;
    let (_user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;
    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app, &email);

    let session_id: (i64,) = sqlx::query_as(
        r#"
        SELECT id
        FROM auth_sessions
        WHERE revoked_at IS NULL
        LIMIT 1
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    let req = test::TestRequest::delete()
        .uri(&format!("/api/auth/sessions/{}", session_id.0))
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .to_request();

    assert_eq!(test::call_service(&app, req).await.status().as_u16(), 200);

    let revoked_at: (Option<String>,) =
        sqlx::query_as("SELECT revoked_at FROM auth_sessions WHERE id = $1")
            .bind(session_id.0)
            .fetch_one(db.pool.as_ref())
            .await
            .unwrap();

    assert!(revoked_at.0.is_some());

    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    assert_eq!(test::call_service(&app, req).await.status().as_u16(), 401);
}

#[actix_web::test]
#[serial]
async fn remember_me_creates_longer_session() {
    let db = setup_test_db().await;
    let (user_id, email) = seed_verified_user(&db, None, "Password123!", "owner").await;
    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": email,
            "password": "Password123!",
            "remember_me": true
        }))
        .to_request();

    assert!(test::call_service(&app, req).await.status().is_success());

    let expires_at: (String,) = sqlx::query_as(
        r#"
        SELECT expires_at
        FROM auth_sessions
        WHERE user_id = $1
        ORDER BY id DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    let expires_at = chrono::DateTime::parse_from_rfc3339(&expires_at.0)
        .unwrap()
        .with_timezone(&Utc);

    assert!(expires_at > Utc::now() + Duration::days(6));
}
