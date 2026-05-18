use actix_web::{HttpResponse, Responder, web};

use crate::db::Db;
use crate::models::engagement_milestone::{
    CreateEngagementMilestone, CreateEngagementMilestoneRequest, EngagementMilestone,
    UpdateEngagementMilestoneRequest,
};
use crate::models::operational_agreement::OperationalAgreement;
use crate::services::event_service::EventService;
use crate::services::notification_email_recipient_service::NotificationRecipientService;
use crate::services::notification_service::NotificationService;
use crate::services::transaction_workflow_service::TransactionWorkflowService;

async fn notify_milestone_parties(
    db: &Db,
    engagement_id: i64,
    milestone_title: String,
    notification_type: &str,
) {
    let agreement =
        match OperationalAgreement::latest_for_engagement(db.pool.as_ref(), engagement_id).await {
            Ok(Some(agreement)) => agreement,
            Ok(None) => {
                eprintln!("No agreement found for engagement {}", engagement_id);
                return;
            }
            Err(err) => {
                eprintln!("latest_for_engagement lookup error: {:?}", err);
                return;
            }
        };

    let emails =
        match NotificationRecipientService::agreement_party_emails(db.pool.as_ref(), agreement.id)
            .await
        {
            Ok(emails) => emails,
            Err(err) => {
                eprintln!("agreement_party_emails lookup error: {:?}", err);
                return;
            }
        };

    for email in emails {
        let (notification_key, title, body) = match notification_type {
            "approved" => (
                "milestone_approved",
                "Milestone approved",
                format!("Milestone approved: {}", milestone_title),
            ),
            "paid" => (
                "milestone_paid",
                "Milestone marked paid",
                format!("Milestone marked paid: {}", milestone_title),
            ),
            _ => continue,
        };

        if let Err(err) = NotificationService::notify_email(
            db.pool.as_ref(),
            agreement.organization_id,
            email,
            notification_key,
            title,
            &body,
            Some("engagement"),
            Some(engagement_id),
        )
        .await
        {
            eprintln!("milestone notification error: {:?}", err);
        }
    }
}
async fn record_milestone_and_engagement_event(
    db: &Db,
    organization_id: i64,
    milestone_id: i64,
    engagement_id: i64,
    event_type: &str,
    to_status: Option<&str>,
    metadata: serde_json::Value,
) {
    let _ = EventService::record_event(
        db.pool.as_ref(),
        organization_id,
        None,
        "milestone",
        milestone_id,
        event_type,
        None,
        to_status,
        metadata.clone(),
    )
    .await;

    let _ = EventService::record_event(
        db.pool.as_ref(),
        organization_id,
        None,
        "engagement",
        engagement_id,
        event_type,
        None,
        to_status,
        metadata,
    )
    .await;
}
async fn organization_id_for_milestone(db: &Db, milestone_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT e.organization_id as "organization_id!"
        FROM engagement_milestones m
        JOIN engagements e ON e.id = m.engagement_id
        WHERE m.id = $1
        "#,
        milestone_id
    )
    .fetch_one(db.pool.as_ref())
    .await
}

async fn engagement_id_for_milestone(db: &Db, milestone_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT engagement_id as "engagement_id!"
        FROM engagement_milestones
        WHERE id = $1
        "#,
        milestone_id
    )
    .fetch_one(db.pool.as_ref())
    .await
}
/// CREATE milestone for engagement
pub async fn create_engagement_milestone(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<CreateEngagementMilestoneRequest>,
) -> impl Responder {
    let engagement_id = path.into_inner();
    let input = payload.into_inner();

    let milestone = CreateEngagementMilestone {
        engagement_id,
        title: input.title,
        description: input.description,
        amount_cents: input.amount_cents.unwrap_or(0),
        due_date: input.due_date,
    };

    match EngagementMilestone::create(db.pool.as_ref(), milestone).await {
        Ok(milestone) => {
            let organization_id = sqlx::query_scalar!(
                r#"
        SELECT organization_id as "organization_id!"
        FROM engagements
        WHERE id = $1
        "#,
                milestone.engagement_id
            )
            .fetch_one(db.pool.as_ref())
            .await
            .unwrap_or(0);

            if organization_id != 0 {
                record_milestone_and_engagement_event(
                    &db,
                    organization_id,
                    milestone.id,
                    milestone.engagement_id,
                    "MilestoneCreated",
                    Some(&milestone.status),
                    serde_json::json!({
                        "milestone_id": milestone.id,
                        "engagement_id": milestone.engagement_id,
                        "title": milestone.title,
                        "amount_cents": milestone.amount_cents
                    }),
                )
                .await;
            }

            HttpResponse::Created().json(milestone)
        }
        Err(err) => {
            eprintln!("create_engagement_milestone error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// LIST milestones for engagement
pub async fn list_engagement_milestones(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    println!("Loading milestones for engagement_id: {}", engagement_id);

    match EngagementMilestone::for_engagement(db.pool.as_ref(), engagement_id).await {
        Ok(items) => HttpResponse::Ok().json(items),

        Err(err) => {
            eprintln!("list_engagement_milestones error: {:?}", err);

            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// MARK milestone submitted
pub async fn submit_engagement_milestone(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let milestone_id = path.into_inner();

    println!("Submitting milestone_id: {}", milestone_id);

    match EngagementMilestone::update_status(db.pool.as_ref(), milestone_id, "submitted").await {
        Ok(item) => {
            if let Ok(organization_id) = organization_id_for_milestone(&db, milestone_id).await {
                {
                    record_milestone_and_engagement_event(
                        &db,
                        organization_id,
                        milestone_id,
                        item.engagement_id,
                        "MilestoneSubmitted",
                        Some("submitted"),
                        serde_json::json!({
                            "milestone_id": milestone_id,
                            "engagement_id": item.engagement_id,
                            "title": item.title
                        }),
                    )
                    .await;
                }
            }

            HttpResponse::Ok().json(item)
        }

        Err(err) => {
            eprintln!("submit_engagement_milestone error: {:?}", err);

            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// MARK milestone approved
pub async fn approve_engagement_milestone(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let milestone_id = path.into_inner();

    println!("Approving milestone_id: {}", milestone_id);

    match EngagementMilestone::update_status(db.pool.as_ref(), milestone_id, "approved").await {
        Ok(item) => {
            if let Ok(organization_id) = organization_id_for_milestone(&db, milestone_id).await {
                record_milestone_and_engagement_event(
                    &db,
                    organization_id,
                    milestone_id,
                    item.engagement_id,
                    "MilestoneApproved",
                    Some("approved"),
                    serde_json::json!({
                        "milestone_id": milestone_id,
                        "engagement_id": item.engagement_id,
                        "title": item.title
                    }),
                )
                .await;
                if let Ok(Some(agreement)) = OperationalAgreement::latest_for_engagement(
                    db.pool.as_ref(),
                    item.engagement_id,
                )
                .await
                {
                    println!("Latest agreement found for milestone approval.");

                    match TransactionWorkflowService::generate_transactions_for_trigger(
                        db.pool.as_ref(),
                        organization_id,
                        agreement.id,
                        Some(item.engagement_id),
                        Some(milestone_id),
                        "MilestoneApproved",
                        item.amount_cents,
                    )
                    .await
                    {
                        Ok(_) => println!("Transaction generation completed."),
                        Err(err) => eprintln!("generate_transactions_for_trigger error: {:?}", err),
                    }
                }
                println!(
                    "Milestone approved: organization_id={}, engagement_id={}, milestone_id={}, amount_cents={}",
                    organization_id, item.engagement_id, milestone_id, item.amount_cents
                );
            }
            notify_milestone_parties(&db, item.engagement_id, item.title.clone(), "approved").await;
            HttpResponse::Ok().json(item)
        }

        Err(err) => {
            eprintln!("approve_engagement_milestone error: {:?}", err);

            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// MARK milestone paid
pub async fn mark_engagement_milestone_paid(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let milestone_id = path.into_inner();

    println!("Marking milestone paid: {}", milestone_id);

    match EngagementMilestone::update_status(db.pool.as_ref(), milestone_id, "paid").await {
        Ok(item) => {
            if let Ok(organization_id) = organization_id_for_milestone(&db, milestone_id).await {
                record_milestone_and_engagement_event(
                    &db,
                    organization_id,
                    milestone_id,
                    item.engagement_id,
                    "MilestonePaid",
                    Some("paid"),
                    serde_json::json!({
                        "milestone_id": milestone_id,
                        "engagement_id": item.engagement_id,
                        "title": item.title,
                        "amount_cents": item.amount_cents
                    }),
                )
                .await;
                notify_milestone_parties(&db, item.engagement_id, item.title.clone(), "paid").await;
            }
            HttpResponse::Ok().json(item)
        }

        Err(err) => {
            eprintln!("mark_engagement_milestone_paid error: {:?}", err);

            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// UPDATE milestone details
pub async fn update_engagement_milestone(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<UpdateEngagementMilestoneRequest>,
) -> impl Responder {
    let milestone_id = path.into_inner();
    let input = payload.into_inner();

    println!("Updating milestone_id: {}", milestone_id);

    match EngagementMilestone::update(db.pool.as_ref(), milestone_id, input).await {
        Ok(item) => {
            if let Ok(organization_id) = organization_id_for_milestone(&db, milestone_id).await {
                record_milestone_and_engagement_event(
                    &db,
                    organization_id,
                    milestone_id,
                    item.engagement_id,
                    "MilestoneUpdated",
                    Some(&item.status),
                    serde_json::json!({
                        "milestone_id": milestone_id,
                        "engagement_id": item.engagement_id,
                        "title": item.title,
                        "amount_cents": item.amount_cents,
                        "due_date": item.due_date
                    }),
                )
                .await;
            }

            HttpResponse::Ok().json(item)
        }

        Err(err) => {
            eprintln!("update_engagement_milestone error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// REOPEN milestone
pub async fn reopen_engagement_milestone(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let milestone_id = path.into_inner();

    println!("Reopening milestone_id: {}", milestone_id);

    match EngagementMilestone::reopen(db.pool.as_ref(), milestone_id).await {
        Ok(item) => {
            if let Ok(organization_id) = organization_id_for_milestone(&db, milestone_id).await {
                record_milestone_and_engagement_event(
                    &db,
                    organization_id,
                    milestone_id,
                    item.engagement_id,
                    "MilestoneReopened",
                    Some(&item.status),
                    serde_json::json!({
                        "milestone_id": milestone_id,
                        "engagement_id": item.engagement_id,
                        "title": item.title,
                        "amount_cents": item.amount_cents,
                        "due_date": item.due_date
                    }),
                )
                .await;
            }

            HttpResponse::Ok().json(item)
        }

        Err(err) => {
            eprintln!("reopen_engagement_milestone error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
