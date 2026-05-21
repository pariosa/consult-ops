// src/auth.rs
use crate::auth_context::AuthUser;
pub mod permissions;
use crate::auth_context::Claims;
use crate::db::Db;
use crate::services::email_notification_service::EmailNotificationService;
use actix_web::{HttpResponse, Responder, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct PasswordResetToken {
    pub id: i64,
    pub user_id: i64,
    pub expires_at: String,
    pub used_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub email: String,
    pub password: String,
    pub remember_me: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterInfo {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub user_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub user_type: String,
    pub email_verified_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub user_id: i64,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct LoginUser {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub user_type: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: LoginUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResendVerificationRequest {
    pub email: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AuthMeResponse {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub user_type: String,
    pub email_verified_at: Option<String>,
    pub active_session_id: Option<i64>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AuthSessionResponse {
    pub id: i64,
    pub created_at: String,
    pub last_seen_at: Option<String>,
    pub expires_at: String,
    pub revoked_at: Option<String>,
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| e.to_string())
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn generate_reset_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

async fn write_audit_event(
    db: &Db,
    actor_user_id: Option<i64>,
    event_type: &str,
    resource_type: Option<&str>,
    resource_id: Option<i64>,
) {
    let _ = sqlx::query(
        r#"
        INSERT INTO audit_events (
            actor_user_id,
            event_type,
            resource_type,
            resource_id,
            metadata_json,
            created_at
        )
        VALUES (?, ?, ?, ?, '{}', ?)
        "#,
    )
    .bind(actor_user_id)
    .bind(event_type)
    .bind(resource_type)
    .bind(resource_id)
    .bind(Utc::now().to_rfc3339())
    .execute(&*db.pool)
    .await;
}

async fn record_auth_attempt(db: &Db, email: &str, action: &str, success: bool) {
    let _ = sqlx::query(
        r#"
        INSERT INTO auth_attempts (
            email,
            action,
            success,
            created_at
        )
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(email)
    .bind(action)
    .bind(if success { 1 } else { 0 })
    .bind(Utc::now().to_rfc3339())
    .execute(&*db.pool)
    .await;
}

async fn too_many_failed_logins(db: &Db, email: &str) -> bool {
    let cutoff = (Utc::now() - Duration::minutes(15)).to_rfc3339();

    let count = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(*)
        FROM auth_attempts
        WHERE email = ?
          AND action = 'login'
          AND success = 0
          AND created_at > ?
        "#,
    )
    .bind(email)
    .bind(cutoff)
    .fetch_one(&*db.pool)
    .await
    .map(|row| row.0)
    .unwrap_or(0);

    count >= 5
}

pub async fn register(db: web::Data<Db>, info: web::Json<RegisterInfo>) -> impl Responder {
    let hash = match hash_password(&info.password) {
        Ok(hash) => hash,
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };

    let now = Utc::now().to_rfc3339();

    let user_type = info
        .user_type
        .clone()
        .unwrap_or_else(|| "owner".to_string());

    let result = sqlx::query(
        r#"
        INSERT INTO users
        (email, password_hash, name, user_type, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&info.email)
    .bind(&hash)
    .bind(&info.name)
    .bind(&user_type)
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await;

    let rec = match result {
        Ok(rec) => rec,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let user_id = rec.last_insert_rowid();

    let raw_token = generate_reset_token();
    let token_hash = hash_token(&raw_token);
    let expires_at = (Utc::now() + Duration::hours(24)).to_rfc3339();

    let token_result = sqlx::query(
        r#"
        INSERT INTO email_verification_tokens
        (user_id, token_hash, expires_at, created_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(user_id)
    .bind(token_hash)
    .bind(expires_at)
    .bind(Utc::now().to_rfc3339())
    .execute(&*db.pool)
    .await;

    if let Err(e) = token_result {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    let app_url = std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let verification_url = format!("{}/verify-email?token={}", app_url, raw_token);

    match EmailNotificationService::email_verification(info.email.clone(), verification_url).await {
        Ok(_) => eprintln!("Verification email sent to {}", info.email),
        Err(err) => eprintln!("EMAIL SEND FAILED: {}", err),
    }
    HttpResponse::Ok().json(AuthResponse {
        message: "User registered".to_string(),
        user_id,
        email: info.email.clone(),
    })
}

pub async fn login(db: web::Data<Db>, info: web::Json<AuthInfo>) -> impl Responder {
    if too_many_failed_logins(&db, &info.email).await {
        return HttpResponse::TooManyRequests().body("Too many failed login attempts");
    }
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, name, user_type, email_verified_at
        FROM users
        WHERE email = ?
    "#,
    )
    .bind(&info.email)
    .fetch_one(&*db.pool)
    .await;

    let user = match user {
        Ok(user) => user,
        Err(_) => {
            record_auth_attempt(&db, &info.email, "login", false).await;
            write_audit_event(&db, None, "auth.login_failed", Some("user"), None).await;

            return HttpResponse::Unauthorized().body("Invalid email or password");
        }
    };
    if user.email_verified_at.is_none() {
        return HttpResponse::Forbidden().body("Email verification required");
    }
    if verify_password(&info.password, &user.password_hash) {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let jti = Uuid::new_v4().to_string();

        let expires_at = if info.remember_me.unwrap_or(false) {
            Utc::now() + Duration::days(30)
        } else {
            Utc::now() + Duration::hours(24)
        };
        let exp = expires_at.timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            user_type: user.user_type.clone(),
            jti: jti.clone(),
            exp,
        };

        let session_result = sqlx::query(
            r#"
        INSERT INTO auth_sessions
        (user_id, token_jti, expires_at, created_at, last_seen_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
        )
        .bind(user.id)
        .bind(&jti)
        .bind(expires_at.to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(&*db.pool)
        .await;

        if let Err(e) = session_result {
            return HttpResponse::InternalServerError().body(e.to_string());
        }

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        ) {
            Ok(token) => token,
            Err(err) => {
                return HttpResponse::InternalServerError().body(err.to_string());
            }
        };
        record_auth_attempt(&db, &user.email, "login", true).await;
        write_audit_event(
            &db,
            Some(user.id),
            "auth.login_success",
            Some("user"),
            Some(user.id),
        )
        .await;
        HttpResponse::Ok().json(LoginResponse {
            token,
            user: LoginUser {
                id: user.id,
                email: user.email,
                name: user.name,
                user_type: user.user_type,
            },
        })
    } else {
        record_auth_attempt(&db, &info.email, "login", false).await;
        write_audit_event(
            &db,
            Some(user.id),
            "auth.login_failed",
            Some("user"),
            Some(user.id),
        )
        .await;

        HttpResponse::Unauthorized().body("Invalid email or password")
    }
}

pub async fn forgot_password(
    db: web::Data<Db>,
    info: web::Json<ForgotPasswordRequest>,
) -> impl Responder {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, name
        FROM users
        WHERE email = ?
        "#,
    )
    .bind(&info.email)
    .fetch_one(&*db.pool)
    .await;

    let user = match user {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Ok().body(
                "If an account exists for this email, a password reset link has been generated.",
            );
        }
    };

    let raw_token = generate_reset_token();
    let token_hash = hash_token(&raw_token);
    let expires_at = (Utc::now() + Duration::minutes(30)).to_rfc3339();

    let result = sqlx::query(
        r#"
    INSERT INTO password_reset_tokens
    (user_id, token_hash, expires_at, created_at)
    VALUES (?, ?, ?, ?)
    "#,
    )
    .bind(user.id)
    .bind(&token_hash)
    .bind(&expires_at)
    .bind(Utc::now().to_rfc3339())
    .execute(&*db.pool)
    .await;

    match result {
        Ok(_) => {
            // Dev-only response. Later, email this token instead.
            let app_url =
                std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

            let reset_url = format!("{}/reset-password?token={}", app_url, raw_token);

            match EmailNotificationService::email_verification(user.email.clone(), reset_url).await
            {
                Ok(_) => eprintln!("Verification email sent to {}", user.email),
                Err(err) => eprintln!("EMAIL SEND FAILED: {}", err),
            }

            HttpResponse::Ok()
                .body("If an account exists for this email, a password reset link has been sent.")
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn reset_password(
    db: web::Data<Db>,
    info: web::Json<ResetPasswordRequest>,
) -> impl Responder {
    let token_hash = hash_token(&info.token);

    let reset = sqlx::query_as::<_, PasswordResetToken>(
        r#"
    SELECT id, user_id, expires_at, used_at
    FROM password_reset_tokens
    WHERE token_hash = ?
    "#,
    )
    .bind(&token_hash)
    .fetch_one(&*db.pool)
    .await;

    let reset = match reset {
        Ok(reset) => reset,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid or expired reset token"),
    };

    if reset.used_at.is_some() {
        return HttpResponse::Unauthorized().body("Reset token already used");
    }

    let expires_at = match chrono::DateTime::parse_from_rfc3339(&reset.expires_at) {
        Ok(date) => date.with_timezone(&Utc),
        Err(_) => return HttpResponse::Unauthorized().body("Invalid reset token"),
    };

    if expires_at < Utc::now() {
        return HttpResponse::Unauthorized().body("Reset token expired");
    }

    let new_hash = match hash_password(&info.password) {
        Ok(hash) => hash,
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };

    let result = sqlx::query(
        r#"
    UPDATE users
    SET password_hash = ?, updated_at = ?
    WHERE id = ?
    "#,
    )
    .bind(&new_hash)
    .bind(Utc::now().to_rfc3339())
    .bind(reset.user_id)
    .execute(&*db.pool)
    .await;

    if let Err(e) = result {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    let _ = sqlx::query(
        r#"
    UPDATE password_reset_tokens
    SET used_at = ?
    WHERE id = ?
    "#,
    )
    .bind(Utc::now().to_rfc3339())
    .bind(reset.id)
    .execute(&*db.pool)
    .await;

    //revoke user sessions after pw reset
    let _ = sqlx::query(
        r#"
    UPDATE auth_sessions
    SET revoked_at = ?
    WHERE user_id = ?
      AND revoked_at IS NULL
    "#,
    )
    .bind(Utc::now().to_rfc3339())
    .bind(reset.user_id)
    .execute(&*db.pool)
    .await;
    write_audit_event(
        &db,
        Some(reset.user_id),
        "auth.password_reset_success",
        Some("user"),
        Some(reset.user_id),
    )
    .await;

    HttpResponse::Ok().body("Password reset successful")
}

pub async fn logout(db: web::Data<Db>, auth_user: AuthUser) -> impl Responder {
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        r#"
        UPDATE auth_sessions
        SET revoked_at = ?
        WHERE user_id = ?
          AND token_jti = ?
          AND revoked_at IS NULL
        "#,
    )
    .bind(&now)
    .bind(auth_user.id)
    .bind(&auth_user.jti)
    .execute(&*db.pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Logged out"
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn verify_email(
    db: web::Data<Db>,
    info: web::Json<VerifyEmailRequest>,
) -> impl Responder {
    let token_hash = hash_token(&info.token);

    let token = sqlx::query_as::<_, PasswordResetToken>(
        r#"
        SELECT id, user_id, expires_at, used_at
        FROM email_verification_tokens
        WHERE token_hash = ?
        "#,
    )
    .bind(token_hash)
    .fetch_one(&*db.pool)
    .await;

    let token = match token {
        Ok(token) => token,
        Err(_) => {
            return HttpResponse::Unauthorized().body("Invalid or expired verification token");
        }
    };

    if token.used_at.is_some() {
        return HttpResponse::Unauthorized().body("Verification token already used");
    }

    let expires_at = match chrono::DateTime::parse_from_rfc3339(&token.expires_at) {
        Ok(date) => date.with_timezone(&Utc),
        Err(_) => return HttpResponse::Unauthorized().body("Invalid verification token"),
    };

    if expires_at < Utc::now() {
        return HttpResponse::Unauthorized().body("Verification token expired");
    }

    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        r#"
        UPDATE users
        SET email_verified_at = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&now)
    .bind(&now)
    .bind(token.user_id)
    .execute(&*db.pool)
    .await;

    if let Err(e) = result {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    let used_result = sqlx::query(
        r#"
        UPDATE email_verification_tokens
        SET used_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&now)
    .bind(token.id)
    .execute(&*db.pool)
    .await;

    if let Err(e) = used_result {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    write_audit_event(
        &db,
        Some(token.user_id),
        "auth.email_verified",
        Some("user"),
        Some(token.user_id),
    )
    .await;

    HttpResponse::Ok().body("Email verified")
}

pub async fn resend_verification(
    db: web::Data<Db>,
    info: web::Json<ResendVerificationRequest>,
) -> impl Responder {
    let generic_message =
        "If an unverified account exists for this email, a verification link has been sent.";

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, name, user_type, email_verified_at
        FROM users
        WHERE email = ?
        "#,
    )
    .bind(&info.email)
    .fetch_optional(&*db.pool)
    .await;

    let Some(user) = (match user {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "message": generic_message
            }));
        }
    }) else {
        return HttpResponse::Ok().json(serde_json::json!({
            "message": generic_message
        }));
    };

    if user.email_verified_at.is_some() {
        return HttpResponse::Ok().json(serde_json::json!({
            "message": generic_message
        }));
    }

    let now = Utc::now().to_rfc3339();

    let _ = sqlx::query(
        r#"
        UPDATE email_verification_tokens
        SET used_at = ?
        WHERE user_id = ?
          AND used_at IS NULL
        "#,
    )
    .bind(&now)
    .bind(user.id)
    .execute(&*db.pool)
    .await;

    let raw_token = generate_reset_token();
    let app_url = std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let verification_url = format!("{}/verify-email?token={}", app_url, raw_token);

    match EmailNotificationService::email_verification(user.email.clone(), verification_url).await {
        Ok(_) => eprintln!("Verification email sent to {}", user.email),
        Err(err) => eprintln!("EMAIL SEND FAILED: {}", err),
    }
    let token_hash = hash_token(&raw_token);
    let expires_at = (Utc::now() + Duration::hours(24)).to_rfc3339();

    let result = sqlx::query(
        r#"
        INSERT INTO email_verification_tokens
        (user_id, token_hash, expires_at, created_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(user.id)
    .bind(token_hash)
    .bind(expires_at)
    .bind(&now)
    .execute(&*db.pool)
    .await;

    if let Err(e) = result {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    write_audit_event(
        &db,
        Some(user.id),
        "auth.verification_resent",
        Some("user"),
        Some(user.id),
    )
    .await;

    HttpResponse::Ok().json(serde_json::json!({
        "message": generic_message
    }))
}
pub async fn list_auth_sessions(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    let result = sqlx::query_as::<_, AuthSessionResponse>(
        r#"
        SELECT id, created_at, last_seen_at, expires_at, revoked_at
        FROM auth_sessions
        WHERE user_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(auth.id)
    .fetch_all(&*db.pool)
    .await;

    match result {
        Ok(sessions) => HttpResponse::Ok().json(sessions),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn revoke_auth_session(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let session_id = path.into_inner();
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        r#"
        UPDATE auth_sessions
        SET revoked_at = ?
        WHERE id = ?
          AND user_id = ?
          AND revoked_at IS NULL
        "#,
    )
    .bind(&now)
    .bind(session_id)
    .bind(auth.id)
    .execute(&*db.pool)
    .await;

    match result {
        Ok(_) => {
            write_audit_event(
                &db,
                Some(auth.id),
                "auth.session_revoked",
                Some("auth_session"),
                Some(session_id),
            )
            .await;

            HttpResponse::Ok().json(serde_json::json!({
                "success": true
            }))
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn auth_me(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    let result = sqlx::query_as::<_, AuthMeResponse>(
        r#"
        SELECT
            u.id,
            u.email,
            u.name,
            u.user_type,
            u.email_verified_at,
            s.id AS active_session_id
        FROM users u
        LEFT JOIN auth_sessions s
            ON s.user_id = u.id
           AND s.token_jti = ?
           AND s.revoked_at IS NULL
        WHERE u.id = ?
        "#,
    )
    .bind(&auth.jti)
    .bind(auth.id)
    .fetch_one(&*db.pool)
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
