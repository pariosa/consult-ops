use crate::db::Db;
use crate::models::project::{CreateProject, CreateProjectRequest, Project};
use crate::services::event_service::EventService;
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

pub async fn create_project(
    db: web::Data<Db>,
    path: web::Path<i64>,
    info: web::Json<CreateProjectRequest>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let input = info.into_inner();

    let new_project = CreateProject {
        organization_id,
        client_id: input.client_id,
        name: input.name,
        start_date: input.start_date.or_else(|| Some(Utc::now().to_rfc3339())),
        description: input.description,
        end_date: input.end_date.or_else(|| Some(Utc::now().to_rfc3339())),
    };

    match Project::create(&db, new_project).await {
        Ok(project) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                project.organization_id,
                None,
                "project",
                project.id,
                "ProjectCreated",
                None,
                Some("created"),
                serde_json::json!({
                    "client_id": project.client_id,
                    "name": project.name
                }),
            )
            .await;

            HttpResponse::Ok().json(project)
        }
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to create project")
        }
    }
}
