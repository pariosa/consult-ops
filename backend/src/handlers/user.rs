use crate::auth::hash_password;
use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::user::{CreateUser, CreateUserRequest, UpdateUserType, User};
use crate::services::authz::require_platform_admin;
use actix_web::{HttpResponse, Responder, ResponseError, web};

pub async fn get_users(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    if let Err(err) = require_platform_admin(&auth) {
        return err.error_response();
    }
    match User::all(&db).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch users")
        }
    }
}

pub async fn get_user_by_id(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if let Err(err) = require_platform_admin(&auth) {
        return err.error_response();
    }
    let user_id = path.into_inner();

    match User::find_by_id(&db, user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

pub async fn update_user_type(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    info: web::Json<UpdateUserType>,
) -> impl Responder {
    if let Err(err) = require_platform_admin(&auth) {
        return err.error_response();
    }

    let user_id = path.into_inner();

    let allowed = ["admin", "consultant", "client"];

    if !allowed.contains(&info.user_type.as_str()) {
        return HttpResponse::BadRequest().body("Invalid user_type");
    }

    match User::update_user_type(&db, user_id, info.user_type.clone()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to update user type")
        }
    }
}

pub async fn create_user(
    db: web::Data<Db>,
    auth: AuthUser,
    info: web::Json<CreateUserRequest>,
) -> impl Responder {
    if let Err(err) = require_platform_admin(&auth) {
        return err.error_response();
    }

    let allowed = ["admin", "consultant", "client"];

    if !allowed.contains(&info.user_type.as_str()) {
        return HttpResponse::BadRequest().body("Invalid user_type");
    }

    let password_hash = match hash_password(&info.password) {
        Ok(hash) => hash,
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };

    let user = CreateUser {
        email: info.email.trim().to_lowercase(),
        password_hash,
        name: info.name.clone(),
        user_type: info.user_type.clone(),
    };

    match User::create(&db, user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::BadRequest().body("Failed to create user")
        }
    }
}
