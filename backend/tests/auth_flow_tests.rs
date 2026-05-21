mod common;

use actix_web::test;
use backend::auth::{hash_password, hash_token};
use chrono::{Duration, Utc};
use common::{setup_test_db, test_app};

async fn seed_verified_user(
    db: &backend::db::Db,
    email: &str,
    password: &str,
    user_type: &str,
) -> i64 {
    let now = chrono::Utc::now().to_rfc3339();
    let password_hash = backend::auth::hash_password(password).unwrap();

    let rec = sqlx::query(
        r#"
        INSERT INTO users
        (email, password_hash, name, user_type, email_verified_at, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(email)
    .bind(user_type)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await
    .unwrap();

    rec.last_insert_rowid()
}
macro_rules! login_and_get_token {
    ($app:expr) => {{
        let req = test::TestRequest::post()
            .uri("/api/auth/login")
            .set_json(serde_json::json!({
                "email": "verified@example.com",
                "password": "Password123!"
            }))
            .to_request();

        let resp: serde_json::Value = test::call_and_read_body_json(&$app, req).await;

        resp["token"]
            .as_str()
            .expect("login response should include token")
            .to_string()
    }};
}

#[actix_web::test]
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
    let status = resp.status();
    let body = test::read_body(resp).await;
    let body_text = String::from_utf8_lossy(&body);

    assert!(
        status.is_success(),
        "expected registration to succeed, got {} body: {}",
        status,
        body_text
    );

    let user: (Option<String>,) = sqlx::query_as::<_, (Option<String>,)>(
        r#"
        SELECT email_verified_at
        FROM users
        WHERE email = ?
        "#,
    )
    .bind("test@example.com")
    .fetch_one(&*db.pool)
    .await
    .expect("expected user to exist");

    assert!(
        user.0.is_none(),
        "newly registered users should start unverified"
    );

    let token_count: (i64,) = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(*)
        FROM email_verification_tokens
        "#,
    )
    .fetch_one(&*db.pool)
    .await
    .expect("expected token count query to work");

    assert_eq!(
        token_count.0, 1,
        "registration should create one email verification token"
    );
}

#[actix_web::test]
async fn login_rejects_unverified_user() {
    let db = setup_test_db().await;
    let app = test::init_service(test_app(db.clone())).await;

    let password_hash = hash_password("Password123!").expect("password should hash");
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO users
        (email, password_hash, name, user_type, email_verified_at, created_at, updated_at)
        VALUES (?, ?, ?, ?, NULL, ?, ?)
        "#,
    )
    .bind("unverified@example.com")
    .bind(password_hash)
    .bind("Unverified User")
    .bind("owner")
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await
    .expect("failed to seed unverified user");

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "unverified@example.com",
            "password": "Password123!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_client_error(),
        "unverified users should not be allowed to log in"
    );

    let session_count: (i64,) = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(*)
        FROM auth_sessions
        "#,
    )
    .fetch_one(&*db.pool)
    .await
    .expect("expected session count query to work");

    assert_eq!(
        session_count.0, 0,
        "unverified login should not create an auth session"
    );
}

#[actix_web::test]
async fn verify_email_accepts_valid_token_once() {
    let db = setup_test_db().await;
    let app = test::init_service(test_app(db.clone())).await;

    let password_hash = hash_password("Password123!").expect("password should hash");
    let now = Utc::now().to_rfc3339();

    let user = sqlx::query(
        r#"
        INSERT INTO users
        (email, password_hash, name, user_type, email_verified_at, created_at, updated_at)
        VALUES (?, ?, ?, ?, NULL, ?, ?)
        "#,
    )
    .bind("verify@example.com")
    .bind(password_hash)
    .bind("Verify User")
    .bind("owner")
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await
    .expect("failed to seed user");

    let user_id = user.last_insert_rowid();

    let raw_token = "valid-email-verification-token";
    let token_hash = hash_token(raw_token);
    let expires_at = (Utc::now() + Duration::minutes(30)).to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO email_verification_tokens
        (user_id, token_hash, expires_at, created_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind(token_hash)
    .bind(expires_at)
    .bind(&now)
    .execute(&*db.pool)
    .await
    .expect("failed to seed email verification token");

    let req = test::TestRequest::post()
        .uri("/api/auth/verify-email")
        .set_json(serde_json::json!({
            "token": raw_token
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(
        resp.status().is_success(),
        "valid verification token should succeed"
    );

    let verified_at: (Option<String>,) = sqlx::query_as::<_, (Option<String>,)>(
        r#"
        SELECT email_verified_at
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(&*db.pool)
    .await
    .expect("expected user verification query to work");

    assert!(
        verified_at.0.is_some(),
        "email_verified_at should be set after verification"
    );

    let used_at: (Option<String>,) = sqlx::query_as::<_, (Option<String>,)>(
        r#"
        SELECT used_at
        FROM email_verification_tokens
        WHERE user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(&*db.pool)
    .await
    .expect("expected token used_at query to work");

    assert!(
        used_at.0.is_some(),
        "verification token should be marked used"
    );

    let second_req = test::TestRequest::post()
        .uri("/api/auth/verify-email")
        .set_json(serde_json::json!({
            "token": raw_token
        }))
        .to_request();

    let second_resp = test::call_service(&app, second_req).await;

    assert!(
        second_resp.status().is_client_error(),
        "verification token should only work once"
    );
}

#[actix_web::test]
async fn forgot_password_never_reveals_whether_email_exists() {
    let db = setup_test_db().await;
    let app = test::init_service(test_app(db.clone())).await;

    let password_hash = hash_password("Password123!").expect("password should hash");
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO users
        (email, password_hash, name, user_type, email_verified_at, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind("existing@example.com")
    .bind(password_hash)
    .bind("Existing User")
    .bind("owner")
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await
    .expect("failed to seed existing user");

    let existing_req = test::TestRequest::post()
        .uri("/api/auth/forgot-password")
        .set_json(serde_json::json!({
            "email": "existing@example.com"
        }))
        .to_request();

    let existing_resp = test::call_service(&app, existing_req).await;
    let existing_status = existing_resp.status();
    let existing_body = test::read_body(existing_resp).await;

    let missing_req = test::TestRequest::post()
        .uri("/api/auth/forgot-password")
        .set_json(serde_json::json!({
            "email": "missing@example.com"
        }))
        .to_request();

    let missing_resp = test::call_service(&app, missing_req).await;
    let missing_status = missing_resp.status();
    let missing_body = test::read_body(missing_resp).await;

    assert_eq!(
        existing_status, missing_status,
        "forgot password should return same status for existing and missing emails"
    );

    assert_eq!(
        existing_body, missing_body,
        "forgot password should return same body for existing and missing emails"
    );
}

#[actix_web::test]
async fn login_creates_active_auth_session() {
    let db = setup_test_db().await;
    seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "verified@example.com",
            "password": "Password123!"
        }))
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert!(resp["token"].as_str().is_some());

    let count: (i64,) =
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM auth_sessions WHERE revoked_at IS NULL")
            .fetch_one(&*db.pool)
            .await
            .unwrap();

    assert_eq!(count.0, 1);
}

#[actix_web::test]
async fn logout_revokes_current_session() {
    let db = setup_test_db().await;
    seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app);
    let req = test::TestRequest::post()
        .uri("/api/auth/logout")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let revoked_count: (i64,) = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM auth_sessions WHERE revoked_at IS NOT NULL",
    )
    .fetch_one(&*db.pool)
    .await
    .unwrap();

    assert_eq!(revoked_count.0, 1);
}

#[actix_web::test]
async fn revoked_session_cannot_access_me() {
    let db = setup_test_db().await;
    seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app);
    let logout_req = test::TestRequest::post()
        .uri("/api/auth/logout")
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .to_request();

    let logout_resp = test::call_service(&app, logout_req).await;
    assert_eq!(logout_resp.status().as_u16(), 200);

    let me_req = test::TestRequest::get()
        .uri("/api/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let me_resp = test::call_service(&app, me_req).await;

    assert_eq!(me_resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn expired_session_cannot_access_me() {
    let db = setup_test_db().await;
    let user_id = seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app);
    sqlx::query(
        r#"
        UPDATE auth_sessions
        SET expires_at = ?
        WHERE user_id = ?
        "#,
    )
    .bind((chrono::Utc::now() - chrono::Duration::hours(1)).to_rfc3339())
    .bind(user_id)
    .execute(&*db.pool)
    .await
    .unwrap();

    let req = test::TestRequest::get()
        .uri("/api/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn password_reset_invalidates_existing_sessions() {
    let db = setup_test_db().await;
    let user_id = seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app);
    let raw_reset_token = "valid-reset-token";
    let token_hash = backend::auth::hash_token(raw_reset_token);
    let expires_at = (chrono::Utc::now() + chrono::Duration::minutes(30)).to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO password_reset_tokens
        (user_id, token_hash, expires_at, created_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind(token_hash)
    .bind(expires_at)
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(&*db.pool)
    .await
    .unwrap();

    let reset_req = test::TestRequest::post()
        .uri("/api/auth/reset-password")
        .set_json(serde_json::json!({
            "token": raw_reset_token,
            "password": "NewPassword123!"
        }))
        .to_request();

    let reset_resp = test::call_service(&app, reset_req).await;
    assert_eq!(reset_resp.status().as_u16(), 200);

    let me_req = test::TestRequest::get()
        .uri("/api/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let me_resp = test::call_service(&app, me_req).await;
    assert_eq!(me_resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn repeated_failed_logins_are_throttled() {
    let db = setup_test_db().await;
    seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;

    for _ in 0..5 {
        let req = test::TestRequest::post()
            .uri("/api/auth/login")
            .set_json(serde_json::json!({
                "email": "verified@example.com",
                "password": "WrongPassword!"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "verified@example.com",
            "password": "WrongPassword!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 429);
}

#[actix_web::test]
async fn successful_login_creates_audit_event() {
    let db = setup_test_db().await;
    seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;
    let _token = login_and_get_token!(app);
    let count: (i64,) = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM audit_events WHERE event_type = 'auth.login_success'",
    )
    .fetch_one(&*db.pool)
    .await
    .unwrap();

    assert_eq!(count.0, 1);
}

#[actix_web::test]
async fn failed_login_creates_audit_event() {
    let db = setup_test_db().await;
    seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "verified@example.com",
            "password": "WrongPassword!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);

    let count: (i64,) = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM audit_events WHERE event_type = 'auth.login_failed'",
    )
    .fetch_one(&*db.pool)
    .await
    .unwrap();

    assert_eq!(count.0, 1);
}

#[actix_web::test]
async fn password_reset_creates_audit_event() {
    let db = setup_test_db().await;
    let user_id = seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;

    let raw_reset_token = "valid-reset-token";
    let token_hash = backend::auth::hash_token(raw_reset_token);

    sqlx::query(
        r#"
        INSERT INTO password_reset_tokens
        (user_id, token_hash, expires_at, created_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind(token_hash)
    .bind((chrono::Utc::now() + chrono::Duration::minutes(30)).to_rfc3339())
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(&*db.pool)
    .await
    .unwrap();

    let req = test::TestRequest::post()
        .uri("/api/auth/reset-password")
        .set_json(serde_json::json!({
            "token": raw_reset_token,
            "password": "NewPassword123!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let count: (i64,) = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM audit_events WHERE event_type = 'auth.password_reset_success'",
    )
    .fetch_one(&*db.pool)
    .await
    .unwrap();

    assert_eq!(count.0, 1);
}

#[actix_web::test]
async fn email_verification_creates_audit_event() {
    let db = setup_test_db().await;

    let password_hash = backend::auth::hash_password("Password123!").unwrap();
    let now = chrono::Utc::now().to_rfc3339();

    let rec = sqlx::query(
        r#"
        INSERT INTO users
        (email, password_hash, name, user_type, email_verified_at, created_at, updated_at)
        VALUES (?, ?, ?, ?, NULL, ?, ?)
        "#,
    )
    .bind("verify-audit@example.com")
    .bind(password_hash)
    .bind("Verify Audit")
    .bind("owner")
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await
    .unwrap();

    let user_id = rec.last_insert_rowid();

    let raw_token = "valid-email-token";
    let token_hash = backend::auth::hash_token(raw_token);

    sqlx::query(
        r#"
        INSERT INTO email_verification_tokens
        (user_id, token_hash, expires_at, created_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind(token_hash)
    .bind((chrono::Utc::now() + chrono::Duration::minutes(30)).to_rfc3339())
    .bind(&now)
    .execute(&*db.pool)
    .await
    .unwrap();

    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/verify-email")
        .set_json(serde_json::json!({
            "token": raw_token
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let count: (i64,) = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM audit_events WHERE event_type = 'auth.email_verified'",
    )
    .fetch_one(&*db.pool)
    .await
    .unwrap();

    assert_eq!(count.0, 1);
}
#[actix_web::test]
async fn auth_me_returns_current_verified_user() {
    let db = setup_test_db().await;
    seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app);

    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp["email"], "verified@example.com");
    assert_eq!(resp["user_type"], "owner");
    assert!(resp["email_verified_at"].as_str().is_some());
    assert!(resp["active_session_id"].as_i64().is_some());
}

#[actix_web::test]
async fn resend_verification_creates_new_token_for_unverified_user() {
    let db = setup_test_db().await;

    let now = chrono::Utc::now().to_rfc3339();
    let password_hash = backend::auth::hash_password("Password123!").unwrap();

    let rec = sqlx::query(
        r#"
        INSERT INTO users
        (email, password_hash, name, user_type, email_verified_at, created_at, updated_at)
        VALUES (?, ?, ?, ?, NULL, ?, ?)
        "#,
    )
    .bind("unverified-resend@example.com")
    .bind(password_hash)
    .bind("Unverified Resend")
    .bind("owner")
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await
    .unwrap();

    let user_id = rec.last_insert_rowid();

    sqlx::query(
        r#"
        INSERT INTO email_verification_tokens
        (user_id, token_hash, expires_at, created_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind(backend::auth::hash_token("old-token"))
    .bind((chrono::Utc::now() + chrono::Duration::hours(24)).to_rfc3339())
    .bind(&now)
    .execute(&*db.pool)
    .await
    .unwrap();

    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/resend-verification")
        .set_json(serde_json::json!({
            "email": "unverified-resend@example.com"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let unused_count: (i64,) = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(*)
        FROM email_verification_tokens
        WHERE user_id = ?
          AND used_at IS NULL
        "#,
    )
    .bind(user_id)
    .fetch_one(&*db.pool)
    .await
    .unwrap();

    assert_eq!(unused_count.0, 1);

    let total_count: (i64,) = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(*)
        FROM email_verification_tokens
        WHERE user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(&*db.pool)
    .await
    .unwrap();

    assert_eq!(total_count.0, 2);
}

#[actix_web::test]
async fn resend_verification_does_not_reveal_missing_email() {
    let db = setup_test_db().await;

    let now = chrono::Utc::now().to_rfc3339();
    let password_hash = backend::auth::hash_password("Password123!").unwrap();

    sqlx::query(
        r#"
        INSERT INTO users
        (email, password_hash, name, user_type, email_verified_at, created_at, updated_at)
        VALUES (?, ?, ?, ?, NULL, ?, ?)
        "#,
    )
    .bind("real-unverified@example.com")
    .bind(password_hash)
    .bind("Real Unverified")
    .bind("owner")
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await
    .unwrap();

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
async fn user_can_list_auth_sessions() {
    let db = setup_test_db().await;
    seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app);

    let req = test::TestRequest::get()
        .uri("/api/auth/sessions")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let sessions: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(sessions.as_array().unwrap().len(), 1);
    assert!(sessions[0]["id"].as_i64().is_some());
    assert!(sessions[0]["expires_at"].as_str().is_some());
    assert!(sessions[0]["revoked_at"].is_null());
}

#[actix_web::test]
async fn user_can_revoke_auth_session() {
    let db = setup_test_db().await;
    seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(app);

    let session_id: (i64,) = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT id
        FROM auth_sessions
        WHERE revoked_at IS NULL
        LIMIT 1
        "#,
    )
    .fetch_one(&*db.pool)
    .await
    .unwrap();

    let req = test::TestRequest::delete()
        .uri(&format!("/api/auth/sessions/{}", session_id.0))
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let revoked_at: (Option<String>,) =
        sqlx::query_as::<_, (Option<String>,)>("SELECT revoked_at FROM auth_sessions WHERE id = ?")
            .bind(session_id.0)
            .fetch_one(&*db.pool)
            .await
            .unwrap();

    assert!(revoked_at.0.is_some());

    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn remember_me_creates_longer_session() {
    let db = setup_test_db().await;
    let user_id = seed_verified_user(&db, "verified@example.com", "Password123!", "owner").await;

    let app = test::init_service(test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "verified@example.com",
            "password": "Password123!",
            "remember_me": true
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let expires_at: (String,) = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT expires_at
        FROM auth_sessions
        WHERE user_id = ?
        ORDER BY id DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_one(&*db.pool)
    .await
    .unwrap();

    let expires_at = chrono::DateTime::parse_from_rfc3339(&expires_at.0)
        .unwrap()
        .with_timezone(&chrono::Utc);

    let minimum_expected = chrono::Utc::now() + chrono::Duration::days(6);

    assert!(
        expires_at > minimum_expected,
        "remember_me session should last longer than standard 24h session"
    );
}
