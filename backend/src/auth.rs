// src/auth.rs
use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use rand::rngs::OsRng;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, PasswordHash, SaltString},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub email: String,
    pub password: String,
}

// REGISTER
pub async fn register(
    pool: web::Data<SqlitePool>,
    info: web::Json<AuthInfo>,
) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = match argon2.hash_password(info.password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let res = sqlx::query!(
        r#"INSERT INTO users (email, password_hash) VALUES (?, ?)"#,
        info.email,
        hash
    )
    .execute(pool.as_ref()) // <--- deref Arc<SqlitePool> correctly
    .await;

    match res {
        Ok(_) => HttpResponse::Ok().body("User registered"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// LOGIN
pub async fn login(
    pool: web::Data<SqlitePool>,
    info: web::Json<AuthInfo>,
) -> impl Responder {
    let rec = sqlx::query!(
        r#"SELECT password_hash FROM users WHERE email = ?"#,
        info.email
    )
    .fetch_one(pool.as_ref()) // <--- deref Arc<SqlitePool>
    .await;

    let rec = match rec {
        Ok(u) => u,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid email or password"),
    };

    let parsed_hash = match PasswordHash::new(&rec.password_hash) {
        Ok(p) => p,
        Err(_) => return HttpResponse::InternalServerError().body("Invalid hash stored"),
    };

    let argon2 = Argon2::default();
    if argon2.verify_password(info.password.as_bytes(), &parsed_hash).is_ok() {
        HttpResponse::Ok().body("Login successful")
    } else {
        HttpResponse::Unauthorized().body("Invalid email or password")
    }
}