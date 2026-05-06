use actix_web::{HttpResponse, Responder, web};

use crate::db::Db;
use crate::models::engagement::{CreateEngagement, CreateEngagementRequest, Engagement};

pub async fn list_for_project(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let project_id = path.into_inner();

    println!("Loading engagements for project_id: {}", project_id);

    match Engagement::for_project(&db.pool, project_id).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => {
            eprintln!("Engagement::for_project error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn create_for_project(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<CreateEngagementRequest>,
) -> impl Responder {
    let project_id = path.into_inner();
    let input = payload.into_inner();

    let organization_id_result: Result<i64, sqlx::Error> =
        sqlx::query_scalar("SELECT organization_id FROM projects WHERE id = ?")
            .bind(project_id)
            .fetch_one(db.pool.as_ref())
            .await;
    let organization_id = match organization_id_result {
        Ok(id) => id,
        Err(err) => {
            eprintln!("Could not find organization for project: {:?}", err);
            return HttpResponse::BadRequest().body("Invalid project_id");
        }
    };

    let engagement = CreateEngagement {
        organization_id,
        project_id,
        contractor_name: input.contractor_name,
        contractor_email: input.contractor_email,
        role: input.role,
        title: input.title,
        scope_of_work: input.scope_of_work,
        deliverables: input.deliverables,
        repo_url: input.repo_url,
        amount_cents: input.amount_cents,
        currency: Some(input.currency.unwrap_or_else(|| "usd".to_string())),
        due_date: input.due_date,
    };

    match Engagement::create(&db.pool, engagement).await {
        Ok(engagement) => HttpResponse::Ok().json(engagement),
        Err(err) => {
            eprintln!("Engagement::create error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn show(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    match Engagement::find(&db.pool, path.into_inner()).await {
        Ok(engagement) => HttpResponse::Ok().json(engagement),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

pub async fn mark_contract_sent(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    match Engagement::update_status(&db.pool, path.into_inner(), "contract_sent").await {
        Ok(engagement) => HttpResponse::Ok().json(engagement),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn mark_signed(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    match Engagement::update_status(&db.pool, path.into_inner(), "contract_signed").await {
        Ok(engagement) => HttpResponse::Ok().json(engagement),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
