use crate::db::Db;
use crate::models::project::{CreateProject, Project};
use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;

pub async fn get_projects(db: web::Data<Db>) -> impl Responder {
    match Project::all(&db).await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch projects")
        }
    }
}

pub async fn create_project(db: web::Data<Db>, info: web::Json<CreateProject>) -> impl Responder {
    let input = info.into_inner();

    let new_project = CreateProject {
        client_id: input.client_id,
        name: input.name,
        start_date: Some(input.start_date.unwrap_or_else(|| Utc::now().to_rfc3339())),
        description: input.description,
        end_date: Some(input.end_date.unwrap_or_else(|| Utc::now().to_rfc3339())),
    };

    match Project::create(&db, new_project).await {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to create project")
        }
    }
}
