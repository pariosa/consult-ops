use crate::db::Db;
use crate::domain::engagement_state::{EngagementEvent, EngagementStatus};
use crate::models::engagement::{CreateEngagement, CreateEngagementRequest, Engagement};
use crate::services::email_notification_service::EmailNotificationService;
use crate::services::event_service::EventService;
use crate::services::notification_email_recipient_service::NotificationRecipientService;
use crate::services::operations_kernel_service::OperationsKernelService;
use actix_web::{HttpResponse, Responder, web};
#[derive(Debug, serde::Serialize)]
struct EngagementLifecycleResponse {
    id: i64,
    organization_id: i64,
    project_id: i64,
    status: String,
}
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
        Ok(engagement) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                engagement.organization_id,
                None,
                "engagement",
                engagement.id,
                "EngagementCreated",
                None,
                Some(&engagement.status),
                serde_json::json!({
                    "project_id": engagement.project_id,
                    "title": engagement.title,
                    "contractor_name": engagement.contractor_name,
                    "contractor_email": engagement.contractor_email,
                    "amount_cents": engagement.amount_cents
                }),
            )
            .await;

            HttpResponse::Ok().json(engagement)
        }
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

pub async fn apply_engagement_lifecycle_event(
    db: web::Data<Db>,
    engagement_id: i64,
    actor_user_id: Option<i64>,
    event: EngagementEvent,
) -> HttpResponse {
    let engagement = match sqlx::query!(
        r#"
        SELECT id, organization_id, status
        FROM engagements
        WHERE id = $1
        "#,
        engagement_id
    )
    .fetch_optional(db.pool.as_ref())
    .await
    {
        Ok(Some(engagement)) => engagement,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Engagement not found"
            }));
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": err.to_string()
            }));
        }
    };

    let current_status: EngagementStatus =
        match serde_json::from_value(serde_json::Value::String(engagement.status.clone())) {
            Ok(status) => status,
            Err(_) => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": format!("Invalid engagement status in database: {}", engagement.status)
                }));
            }
        };

    let next_status = match OperationsKernelService::apply_engagement_event(
        db.pool.as_ref(),
        engagement.organization_id,
        engagement.id,
        actor_user_id,
        current_status,
        event,
    )
    .await
    {
        Ok(status) => status,
        Err(err) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": err
            }));
        }
    };

    let next_status_string = serde_json::to_value(next_status)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", next_status).to_lowercase());

    let updated = sqlx::query_as!(
        EngagementLifecycleResponse,
        r#"
        UPDATE engagements
        SET status = $1
        WHERE id = $2
        RETURNING
            id as "id!",
            organization_id as "organization_id!",
            project_id as "project_id!",
            status as "status!"
        "#,
        next_status_string,
        engagement_id
    )
    .fetch_one(db.pool.as_ref())
    .await;

    match updated {
        Ok(engagement) => HttpResponse::Ok().json(engagement),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
pub async fn mark_contract_sent(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let engagement_id: i64 = path.into_inner();

    let response = apply_engagement_lifecycle_event(
        db.clone(),
        engagement_id,
        None,
        EngagementEvent::ContractSent,
    )
    .await;

    if let Ok(engagement) = sqlx::query!(
        r#"
        SELECT title
        FROM engagements
        WHERE id = ?
        "#,
        engagement_id
    )
    .fetch_one(db.pool.as_ref())
    .await
    {
        if let Ok(Some(client_email)) =
            NotificationRecipientService::engagement_client_email(db.pool.as_ref(), engagement_id)
                .await
        {
            if let Err(err) =
                EmailNotificationService::contract_sent(client_email, engagement.title).await
            {
                eprintln!("contract sent email error: {:?}", err);
            }
        }
    }

    response
}

pub async fn mark_signed(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    let response = apply_engagement_lifecycle_event(
        db.clone(),
        engagement_id,
        None,
        EngagementEvent::ContractSigned,
    )
    .await;

    if let Ok(engagement) = sqlx::query!(
        r#"
        SELECT title
        FROM engagements
        WHERE id = ?
        "#,
        engagement_id
    )
    .fetch_one(db.pool.as_ref())
    .await
    {
        if let Ok(Some(client_email)) =
            NotificationRecipientService::engagement_client_email(db.pool.as_ref(), engagement_id)
                .await
        {
            if let Err(err) =
                EmailNotificationService::contract_signed(client_email, engagement.title).await
            {
                eprintln!("contract signed email error: {:?}", err);
            }
        }
    }

    response
}

pub async fn activate_engagement(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    let response = apply_engagement_lifecycle_event(
        db.clone(),
        engagement_id,
        None,
        EngagementEvent::PaymentReceived,
    )
    .await;

    if let Ok(Some(client_email)) =
        NotificationRecipientService::engagement_client_email(db.pool.as_ref(), engagement_id).await
    {
        if let Err(err) = EmailNotificationService::billing_paid(
            client_email,
            format!("engagement {}", engagement_id),
        )
        .await
        {
            eprintln!("activation email error: {:?}", err);
        }
    }

    response
}

pub async fn complete_engagement(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    apply_engagement_lifecycle_event(db, engagement_id, None, EngagementEvent::Complete).await
}

pub async fn cancel_engagement(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    apply_engagement_lifecycle_event(db, engagement_id, None, EngagementEvent::Cancel).await
}

pub async fn dispute_engagement(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    apply_engagement_lifecycle_event(db, engagement_id, None, EngagementEvent::Dispute).await
}
