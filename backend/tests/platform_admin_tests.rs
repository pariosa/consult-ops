mod common;

use actix_web::test;
use backend::auth::hash_password;
use chrono::Utc;
use common::{setup_test_db, test_app};

async fn seed_verified_user(
    db: &backend::db::Db,
    email: &str,
    password: &str,
    name: &str,
    user_type: &str,
) -> i64 {
    let now = Utc::now().to_rfc3339();
    let password_hash = hash_password(password).unwrap();

    let rec = sqlx::query(
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
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(name)
    .bind(user_type)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await
    .unwrap();

    rec.last_insert_rowid()
}

async fn seed_organization(db: &backend::db::Db, name: &str) -> i64 {
    let now = Utc::now().to_rfc3339();

    let rec = sqlx::query(
        r#"
        INSERT INTO organizations (
            name,
            created_at,
            updated_at
        )
        VALUES (?, ?, ?)
        "#,
    )
    .bind(name)
    .bind(&now)
    .bind(&now)
    .execute(&*db.pool)
    .await
    .unwrap();

    rec.last_insert_rowid()
}

#[actix_web::test]
async fn super_admin_can_create_and_list_platform_organizations() {
    let db = setup_test_db().await;

    seed_verified_user(
        &db,
        "root@atlas.test",
        "DemoPass123!",
        "Root Admin",
        "super_admin",
    )
    .await;

    let app = test::init_service(test_app(db.clone())).await;

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "root@atlas.test",
            "password": "DemoPass123!"
        }))
        .to_request();

    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;
    let token = login_resp["token"].as_str().unwrap().to_string();

    let create_req = test::TestRequest::post()
        .uri("/api/platform/organizations")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "Atlas Operations"
        }))
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status().as_u16(), 201);

    let list_req = test::TestRequest::get()
        .uri("/api/platform/organizations")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let orgs: serde_json::Value = test::call_and_read_body_json(&app, list_req).await;

    assert!(
        orgs.as_array()
            .unwrap()
            .iter()
            .any(|org| { org["name"] == "Atlas Operations" })
    );
}

#[actix_web::test]
async fn non_super_admin_cannot_create_platform_organization() {
    let db = setup_test_db().await;

    seed_verified_user(
        &db,
        "admin@atlas.test",
        "DemoPass123!",
        "Org Admin",
        "admin",
    )
    .await;

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

    let create_req = test::TestRequest::post()
        .uri("/api/platform/organizations")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "Blocked Org"
        }))
        .to_request();

    let resp = test::call_service(&app, create_req).await;

    assert_eq!(resp.status().as_u16(), 403);
}

#[actix_web::test]
async fn super_admin_can_create_and_list_platform_users() {
    let db = setup_test_db().await;

    seed_verified_user(
        &db,
        "root@atlas.test",
        "DemoPass123!",
        "Root Admin",
        "super_admin",
    )
    .await;

    let app = test::init_service(test_app(db.clone())).await;

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "root@atlas.test",
            "password": "DemoPass123!"
        }))
        .to_request();

    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;
    let token = login_resp["token"].as_str().unwrap().to_string();

    let create_req = test::TestRequest::post()
        .uri("/api/platform/users")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "email": "new-user@atlas.test",
            "name": "New User",
            "user_type": "operations_manager",
            "password": "DemoPass123!"
        }))
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status().as_u16(), 201);

    let list_req = test::TestRequest::get()
        .uri("/api/platform/users")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let users: serde_json::Value = test::call_and_read_body_json(&app, list_req).await;

    assert!(
        users
            .as_array()
            .unwrap()
            .iter()
            .any(|user| { user["email"] == "new-user@atlas.test" })
    );
}

#[actix_web::test]
async fn super_admin_can_assign_user_to_organization() {
    let db = setup_test_db().await;

    seed_verified_user(
        &db,
        "root@atlas.test",
        "DemoPass123!",
        "Root Admin",
        "super_admin",
    )
    .await;

    let user_id = seed_verified_user(
        &db,
        "member@atlas.test",
        "DemoPass123!",
        "Member User",
        "operations_manager",
    )
    .await;

    let organization_id = seed_organization(&db, "Atlas Client Org").await;

    let app = test::init_service(test_app(db.clone())).await;

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "root@atlas.test",
            "password": "DemoPass123!"
        }))
        .to_request();

    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;
    let token = login_resp["token"].as_str().unwrap().to_string();

    let assign_req = test::TestRequest::post()
        .uri(&format!(
            "/api/platform/organizations/{}/members",
            organization_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "user_id": user_id,
            "role": "operations_manager"
        }))
        .to_request();

    let assign_resp = test::call_service(&app, assign_req).await;
    assert_eq!(assign_resp.status().as_u16(), 200);

    let list_req = test::TestRequest::get()
        .uri(&format!(
            "/api/platform/organizations/{}/members",
            organization_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let members: serde_json::Value = test::call_and_read_body_json(&app, list_req).await;

    assert_eq!(members.as_array().unwrap().len(), 1);
    assert_eq!(members[0]["user_id"], user_id);
    assert_eq!(members[0]["role"], "operations_manager");
    assert_eq!(members[0]["status"], "active");
}

#[actix_web::test]
async fn assigning_same_user_updates_existing_membership_role() {
    let db = setup_test_db().await;

    seed_verified_user(
        &db,
        "root@atlas.test",
        "DemoPass123!",
        "Root Admin",
        "super_admin",
    )
    .await;

    let user_id = seed_verified_user(
        &db,
        "member@atlas.test",
        "DemoPass123!",
        "Member User",
        "operations_manager",
    )
    .await;

    let organization_id = seed_organization(&db, "Atlas Client Org").await;

    let app = test::init_service(test_app(db.clone())).await;

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": "root@atlas.test",
            "password": "DemoPass123!"
        }))
        .to_request();

    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;
    let token = login_resp["token"].as_str().unwrap().to_string();

    let first_assign_req = test::TestRequest::post()
        .uri(&format!(
            "/api/platform/organizations/{}/members",
            organization_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "user_id": user_id,
            "role": "operations_manager"
        }))
        .to_request();

    let first_resp = test::call_service(&app, first_assign_req).await;
    assert_eq!(first_resp.status().as_u16(), 200);

    let second_assign_req = test::TestRequest::post()
        .uri(&format!(
            "/api/platform/organizations/{}/members",
            organization_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "user_id": user_id,
            "role": "finance_admin"
        }))
        .to_request();

    let second_resp = test::call_service(&app, second_assign_req).await;
    assert_eq!(second_resp.status().as_u16(), 200);

    let list_req = test::TestRequest::get()
        .uri(&format!(
            "/api/platform/organizations/{}/members",
            organization_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let members: serde_json::Value = test::call_and_read_body_json(&app, list_req).await;

    assert_eq!(members.as_array().unwrap().len(), 1);
    assert_eq!(members[0]["user_id"], user_id);
    assert_eq!(members[0]["role"], "finance_admin");
    assert_eq!(members[0]["status"], "active");
}
