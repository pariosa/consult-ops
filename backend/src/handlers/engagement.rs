use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::domain::engagement_state::{EngagementEvent, EngagementStatus};
use crate::models::engagement::{CreateEngagement, CreateEngagementRequest, Engagement};
use crate::services::authz::{require_org_member, require_org_role};
use crate::services::email_notification_service::EmailNotificationService;
use crate::services::event_service::EventService;
use crate::services::notification_email_recipient_service::NotificationRecipientService;
use crate::services::notification_service::NotificationService;
use crate::services::operations_kernel_service::OperationsKernelService;
use actix_web::{HttpResponse, Responder, ResponseError, web};

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
struct EngagementLifecycleResponse {
    id: i64,
    organization_id: i64,
    project_id: i64,
    status: String,
}

#[derive(Debug, sqlx::FromRow)]
struct EngagementNotificationContext {
    title: String,
    organization_id: i64,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateContractRecipientRequest {
    pub contractor_email: String,
}
async fn engagement_email_context(
    db: &Db,
    engagement_id: i64,
) -> Result<(String, String), sqlx::Error> {
    sqlx::query_as::<_, (String, String)>(
        r#"
        SELECT contractor_email, title
        FROM engagements
        WHERE id = $1
        "#,
    )
    .bind(engagement_id)
    .fetch_one(db.pool.as_ref())
    .await
}
pub async fn resend_contract(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    let engagement = match Engagement::find_for_user(db.pool.as_ref(), engagement_id, auth.id).await
    {
        Ok(engagement) => engagement,
        Err(_) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Engagement not found"
            }));
        }
    };

    if let Err(err) = require_org_role(
        db.pool.as_ref(),
        auth.id,
        engagement.organization_id,
        &["owner", "admin"],
    )
    .await
    {
        return err.error_response();
    }

    if [
        "contract_signed",
        "active",
        "paid",
        "completed",
        "cancelled",
    ]
    .contains(&engagement.status.as_str())
    {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Contract resend is locked after signing or activation"
        }));
    }

    if let Err(err) = EmailNotificationService::contract_sent(
        engagement.contractor_email.clone(),
        engagement.title.clone(),
    )
    .await
    {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to send contract email: {}", err)
        }));
    }

    let _ = EventService::record_event(
        db.pool.as_ref(),
        engagement.organization_id,
        Some(auth.id),
        "engagement",
        engagement.id,
        "EngagementContractResent",
        Some(&engagement.status),
        Some(&engagement.status),
        serde_json::json!({
            "contractor_email": engagement.contractor_email,
            "title": engagement.title
        }),
    )
    .await;

    notify_client(
        &db,
        engagement_id,
        "contract_resent",
        "Contract resent for review",
        "A contract has been resent for review",
    )
    .await;

    HttpResponse::Ok().json(serde_json::json!({
        "success": true
    }))
}

pub async fn update_contract_recipient(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<UpdateContractRecipientRequest>,
) -> impl Responder {
    let engagement_id = path.into_inner();
    let input = payload.into_inner();

    let organization_id = match organization_id_for_engagement(&db, engagement_id).await {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Engagement not found"
            }));
        }
    };

    if let Err(err) = require_org_role(
        db.pool.as_ref(),
        auth.id,
        organization_id,
        &["owner", "admin"],
    )
    .await
    {
        return err.error_response();
    }

    let current_status: String = match sqlx::query_scalar(
        r#"
        SELECT status
        FROM engagements
        WHERE id = $1
        "#,
    )
    .bind(engagement_id)
    .fetch_one(db.pool.as_ref())
    .await
    {
        Ok(status) => status,
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": err.to_string()
            }));
        }
    };

    if [
        "contract_signed",
        "active",
        "paid",
        "completed",
        "cancelled",
    ]
    .contains(&current_status.as_str())
    {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Contract recipient is locked after signing or activation"
        }));
    }

    let updated = sqlx::query_as::<_, Engagement>(
        r#"
        UPDATE engagements
        SET contractor_email = $1,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $2
        RETURNING *
        "#,
    )
    .bind(input.contractor_email)
    .bind(engagement_id)
    .fetch_one(db.pool.as_ref())
    .await;

    match updated {
        Ok(engagement) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                engagement.organization_id,
                Some(auth.id),
                "engagement",
                engagement.id,
                "EngagementContractRecipientUpdated",
                Some(&current_status),
                Some(&engagement.status),
                serde_json::json!({
                    "contractor_email": engagement.contractor_email
                }),
            )
            .await;

            HttpResponse::Ok().json(engagement)
        }
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
async fn organization_id_for_project(db: &Db, project_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT organization_id
        FROM projects
        WHERE id = $1
        "#,
    )
    .bind(project_id)
    .fetch_one(db.pool.as_ref())
    .await
}

async fn organization_id_for_engagement(db: &Db, engagement_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT organization_id
        FROM engagements
        WHERE id = $1
        "#,
    )
    .bind(engagement_id)
    .fetch_one(db.pool.as_ref())
    .await
}

async fn notification_context_for_engagement(
    db: &Db,
    engagement_id: i64,
) -> Result<EngagementNotificationContext, sqlx::Error> {
    sqlx::query_as::<_, EngagementNotificationContext>(
        r#"
        SELECT title, organization_id
        FROM engagements
        WHERE id = $1
        "#,
    )
    .bind(engagement_id)
    .fetch_one(db.pool.as_ref())
    .await
}

pub async fn list_for_project(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let project_id = path.into_inner();

    let organization_id = match organization_id_for_project(&db, project_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Project not found"),
    };

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match Engagement::for_project(db.pool.as_ref(), project_id).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => {
            eprintln!("Engagement::for_project error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn create_for_project(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<CreateEngagementRequest>,
) -> impl Responder {
    let project_id = path.into_inner();
    let input = payload.into_inner();

    let organization_id = match organization_id_for_project(&db, project_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Project not found"),
    };

    if let Err(err) = require_org_role(
        db.pool.as_ref(),
        auth.id,
        organization_id,
        &["owner", "admin"],
    )
    .await
    {
        return err.error_response();
    }

    let engagement = CreateEngagement {
        organization_id,
        project_id,
        engagement_type: input.engagement_type,
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

    match Engagement::create(db.pool.as_ref(), engagement).await {
        Ok(engagement) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                engagement.organization_id,
                Some(auth.id),
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

            HttpResponse::Created().json(engagement)
        }
        Err(err) => {
            eprintln!("Engagement::create error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn show(db: web::Data<Db>, auth: AuthUser, path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    match Engagement::find_for_user(db.pool.as_ref(), engagement_id, auth.id).await {
        Ok(engagement) => HttpResponse::Ok().json(engagement),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

pub async fn apply_engagement_lifecycle_event(
    db: web::Data<Db>,
    auth: AuthUser,
    engagement_id: i64,
    event: EngagementEvent,
) -> HttpResponse {
    let organization_id = match organization_id_for_engagement(&db, engagement_id).await {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Engagement not found"
            }));
        }
    };

    if let Err(err) = require_org_role(
        db.pool.as_ref(),
        auth.id,
        organization_id,
        &["owner", "admin"],
    )
    .await
    {
        return err.error_response();
    }

    let engagement = match sqlx::query_as::<_, EngagementLifecycleResponse>(
        r#"
        SELECT id, organization_id, project_id, status
        FROM engagements
        WHERE id = $1
        "#,
    )
    .bind(engagement_id)
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
        Some(auth.id),
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

    let updated = sqlx::query_as::<_, EngagementLifecycleResponse>(
        r#"
        UPDATE engagements
        SET status = $1,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $2
        RETURNING id, organization_id, project_id, status
        "#,
    )
    .bind(next_status_string)
    .bind(engagement_id)
    .fetch_one(db.pool.as_ref())
    .await;

    match updated {
        Ok(engagement) => HttpResponse::Ok().json(engagement),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

async fn notify_client(
    db: &Db,
    engagement_id: i64,
    notification_key: &str,
    title: &str,
    body_prefix: &str,
) {
    let context = match notification_context_for_engagement(db, engagement_id).await {
        Ok(context) => context,
        Err(err) => {
            eprintln!("engagement notification context error: {:?}", err);
            return;
        }
    };

    match NotificationRecipientService::engagement_client_email(db.pool.as_ref(), engagement_id)
        .await
    {
        Ok(Some(client_email)) => {
            if let Err(err) = NotificationService::notify_email(
                db.pool.as_ref(),
                context.organization_id,
                client_email,
                notification_key,
                title,
                &format!("{}: {}", body_prefix, context.title),
                Some("engagement"),
                Some(engagement_id),
            )
            .await
            {
                eprintln!("engagement notification error: {:?}", err);
            }
        }
        Ok(None) => {}
        Err(err) => eprintln!("engagement_client_email lookup error: {:?}", err),
    }
}

pub async fn mark_contract_sent(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    let response = apply_engagement_lifecycle_event(
        db.clone(),
        auth,
        engagement_id,
        EngagementEvent::ContractSent,
    )
    .await;

    if response.status().is_success() {
        if response.status().is_success() {
            if let Ok((contractor_email, title)) =
                engagement_email_context(&db, engagement_id).await
            {
                if let Err(err) =
                    EmailNotificationService::contract_sent(contractor_email, title).await
                {
                    eprintln!("contract sent email error: {:?}", err);
                }
            }

            notify_client(
                &db,
                engagement_id,
                "contract_sent",
                "Contract sent for review",
                "A contract has been sent for review",
            )
            .await;
        }
    }

    response
}

pub async fn mark_signed(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    let response = apply_engagement_lifecycle_event(
        db.clone(),
        auth,
        engagement_id,
        EngagementEvent::ContractSigned,
    )
    .await;

    if response.status().is_success() {
        if let Ok((contractor_email, title)) = engagement_email_context(&db, engagement_id).await {
            if let Err(err) =
                EmailNotificationService::contract_signed(contractor_email, title).await
            {
                eprintln!("contract signed email error: {:?}", err);
            }
        }

        notify_client(
            &db,
            engagement_id,
            "contract_signed",
            "Contract signed",
            "A contract has been signed",
        )
        .await;
    }
    response
}

pub async fn activate_engagement(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    let response = apply_engagement_lifecycle_event(
        db.clone(),
        auth,
        engagement_id,
        EngagementEvent::PaymentReceived,
    )
    .await;

    if response.status().is_success() {
        if let Ok(Some(client_email)) =
            NotificationRecipientService::engagement_client_email(db.pool.as_ref(), engagement_id)
                .await
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
    }

    response
}

pub async fn complete_engagement(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    let organization_id = match organization_id_for_engagement(&db, engagement_id).await {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Engagement not found"
            }));
        }
    };

    if let Err(err) = require_org_role(
        db.pool.as_ref(),
        auth.id,
        organization_id,
        &["owner", "admin"],
    )
    .await
    {
        return err.error_response();
    }

    let open_milestone_count: i64 = match sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagement_milestones
        WHERE engagement_id = $1
          AND status NOT IN ('paid', 'completed')
        "#,
    )
    .bind(engagement_id)
    .fetch_one(db.pool.as_ref())
    .await
    {
        Ok(count) => count,
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": err.to_string()
            }));
        }
    };

    if open_milestone_count > 0 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "All milestones must be paid or completed before closing this engagement"
        }));
    }

    let response = apply_engagement_lifecycle_event(
        db.clone(),
        auth,
        engagement_id,
        EngagementEvent::Complete,
    )
    .await;

    if response.status().is_success() {
        if let Ok((contractor_email, title)) = engagement_email_context(&db, engagement_id).await {
            if let Err(err) =
                EmailNotificationService::engagement_completed(contractor_email, title).await
            {
                eprintln!("engagement completed email error: {:?}", err);
            }
        }
    }

    response
}
pub async fn cancel_engagement(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    apply_engagement_lifecycle_event(db, auth, engagement_id, EngagementEvent::Cancel).await
}

pub async fn dispute_engagement(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    apply_engagement_lifecycle_event(db, auth, engagement_id, EngagementEvent::Dispute).await
}
