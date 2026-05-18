use sqlx::{Result as SqlxResult, SqlitePool};

use crate::services::email_notification_service::{EmailMessage, EmailNotificationService};

pub struct NotificationService;

impl NotificationService {
    pub async fn create_notification(
        db: &SqlitePool,
        organization_id: i64,
        user_id: Option<i64>,
        recipient_email: Option<String>,
        notification_type: &str,
        title: &str,
        body: &str,
        entity_type: Option<&str>,
        entity_id: Option<i64>,
    ) -> SqlxResult<i64> {
        let notification_id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO notifications (
                organization_id,
                user_id,
                recipient_email,
                notification_type,
                title,
                body,
                entity_type,
                entity_id,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))
            RETURNING id
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .bind(recipient_email)
        .bind(notification_type)
        .bind(title)
        .bind(body)
        .bind(entity_type)
        .bind(entity_id)
        .fetch_one(db)
        .await?;

        Ok(notification_id)
    }

    pub async fn create_email_job(db: &SqlitePool, notification_id: i64) -> SqlxResult<i64> {
        sqlx::query_scalar(
            r#"
            INSERT INTO notification_jobs (
                notification_id,
                channel,
                status,
                attempts,
                run_after,
                created_at,
                updated_at
            )
            VALUES (?, 'email', 'pending', 0, datetime('now'), datetime('now'), datetime('now'))
            RETURNING id
            "#,
        )
        .bind(notification_id)
        .fetch_one(db)
        .await
    }

    pub async fn notify_email(
        db: &SqlitePool,
        organization_id: i64,
        recipient_email: String,
        notification_type: &str,
        title: &str,
        body: &str,
        entity_type: Option<&str>,
        entity_id: Option<i64>,
    ) -> Result<(), String> {
        let notification_id = Self::create_notification(
            db,
            organization_id,
            None,
            Some(recipient_email.clone()),
            notification_type,
            title,
            body,
            entity_type,
            entity_id,
        )
        .await
        .map_err(|err| err.to_string())?;

        Self::create_email_job(db, notification_id)
            .await
            .map_err(|err| err.to_string())?;

        // Temporary inline send until queue worker exists.
        EmailNotificationService::send(EmailMessage {
            to: recipient_email,
            subject: title.to_string(),
            body: body.to_string(),
        })
        .await?;

        Ok(())
    }
}
