use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub body: String,
}

pub struct EmailNotificationService;

impl EmailNotificationService {
    pub async fn send(message: EmailMessage) -> Result<(), String> {
        let mode = std::env::var("EMAIL_MODE").unwrap_or_else(|_| "dev".to_string());

        if mode == "dev" {
            println!("\n========== DEV EMAIL ==========");
            println!("TO: {}", message.to);
            println!("SUBJECT: {}", message.subject);
            println!("BODY:\n{}", message.body);
            println!("===============================\n");
            return Ok(());
        }

        Err("No production email provider configured.".to_string())
    }

    pub async fn invitation(to: String, role: String, invite_url: String) -> Result<(), String> {
        Self::send(EmailMessage {
            to,
            subject: "You're invited to join Consult Ops".to_string(),
            body: format!(
                "You've been invited to join Consult Ops as {}.\n\nAccept your invitation here:\n{}",
                role, invite_url
            ),
        })
        .await
    }

    pub async fn invitation_accepted(
        to: String,
        accepted_email: String,
        role: String,
    ) -> Result<(), String> {
        Self::send(EmailMessage {
            to,
            subject: "Organization invitation accepted".to_string(),
            body: format!("{} accepted their invitation as {}.", accepted_email, role),
        })
        .await
    }

    pub async fn contract_sent(to: String, title: String) -> Result<(), String> {
        Self::send(EmailMessage {
            to,
            subject: "Contract sent for review".to_string(),
            body: format!("A contract has been sent for review: {}", title),
        })
        .await
    }

    pub async fn contract_signed(to: String, title: String) -> Result<(), String> {
        Self::send(EmailMessage {
            to,
            subject: "Contract signed".to_string(),
            body: format!("A contract has been signed: {}", title),
        })
        .await
    }

    pub async fn activation_checkout(to: String, checkout_url: String) -> Result<(), String> {
        Self::send(EmailMessage {
            to,
            subject: "Activation payment required".to_string(),
            body: format!("Complete your activation payment here:\n{}", checkout_url),
        })
        .await
    }

    pub async fn billing_paid(to: String, label: String) -> Result<(), String> {
        Self::send(EmailMessage {
            to,
            subject: "Payment received".to_string(),
            body: format!("Payment received for {}.", label),
        })
        .await
    }

    pub async fn milestone_approved(to: String, title: String) -> Result<(), String> {
        Self::send(EmailMessage {
            to,
            subject: "Milestone approved".to_string(),
            body: format!("Milestone approved: {}", title),
        })
        .await
    }

    pub async fn milestone_paid(to: String, title: String) -> Result<(), String> {
        Self::send(EmailMessage {
            to,
            subject: "Milestone marked paid".to_string(),
            body: format!("Milestone marked paid: {}", title),
        })
        .await
    }

    pub async fn transaction_paid(to: String, amount_cents: i64) -> Result<(), String> {
        Self::send(EmailMessage {
            to,
            subject: "Transaction marked paid".to_string(),
            body: format!(
                "A transaction for ${:.2} was marked paid.",
                amount_cents as f64 / 100.0
            ),
        })
        .await
    }

    pub async fn transaction_failed(to: String, amount_cents: i64) -> Result<(), String> {
        Self::send(EmailMessage {
            to,
            subject: "Transaction failed".to_string(),
            body: format!(
                "A transaction for ${:.2} failed.",
                amount_cents as f64 / 100.0
            ),
        })
        .await
    }
}
