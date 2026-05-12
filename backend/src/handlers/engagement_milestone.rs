use actix_web::{HttpResponse, Responder, web};

use crate::db::Db;
use crate::models::engagement_milestone::{
    CreateEngagementMilestone, CreateEngagementMilestoneRequest, EngagementMilestone,
    UpdateEngagementMilestoneRequest,
};
use crate::services::event_service::EventService;

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
                let _ = EventService::record_event(
                    db.pool.as_ref(),
                    organization_id,
                    None,
                    "milestone",
                    milestone.id,
                    "MilestoneCreated",
                    None,
                    Some(&milestone.status),
                    serde_json::json!({
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
                let _ = EventService::record_event(
                    db.pool.as_ref(),
                    organization_id,
                    None,
                    "milestone",
                    milestone_id,
                    "MilestoneSubmitted",
                    None,
                    Some("submitted"),
                    serde_json::json!({
                        "engagement_id": item.engagement_id
                    }),
                )
                .await;
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
                let _ = EventService::record_event(
                    db.pool.as_ref(),
                    organization_id,
                    None,
                    "milestone",
                    milestone_id,
                    "MilestoneApproved",
                    None,
                    Some("approved"),
                    serde_json::json!({
                        "engagement_id": item.engagement_id
                    }),
                )
                .await;
            }

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
                let _ = EventService::record_event(
                    db.pool.as_ref(),
                    organization_id,
                    None,
                    "milestone",
                    milestone_id,
                    "MilestonePaid",
                    None,
                    Some("paid"),
                    serde_json::json!({
                        "engagement_id": item.engagement_id
                    }),
                )
                .await;
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
        Ok(item) => HttpResponse::Ok().json(item),

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
                let _ = EventService::record_event(
                    db.pool.as_ref(),
                    organization_id,
                    None,
                    "milestone",
                    milestone_id,
                    "MilestoneReopened",
                    None,
                    Some(&item.status),
                    serde_json::json!({
                        "engagement_id": item.engagement_id
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
