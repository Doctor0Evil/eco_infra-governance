use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditKind {
    ValidationSuccess,
    ValidationFailed { reason: String },
    RoutingAllowed { label: String },
    RoutingDenied { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: Uuid,
    pub session_id: Uuid,
    pub timestamp_utc: DateTime<Utc>,
    pub kind: AuditKind,
}

impl AuditEvent {
    pub fn validation_success() -> Self {
        Self::simple(AuditKind::ValidationSuccess)
    }

    pub fn validation_failed(reason: &str) -> Self {
        Self::simple(AuditKind::ValidationFailed {
            reason: reason.to_string(),
        })
    }

    pub fn routing_allowed(label: &str) -> Self {
        Self::simple(AuditKind::RoutingAllowed {
            label: label.to_string(),
        })
    }

    pub fn routing_denied(reason: &str) -> Self {
        Self::simple(AuditKind::RoutingDenied {
            reason: reason.to_string(),
        })
    }

    fn simple(kind: AuditKind) -> Self {
        AuditEvent {
            event_id: Uuid::new_v4(),
            session_id: Uuid::new_v4(), // in production, bind to caller’s session token. [file:1][file:5]
            timestamp_utc: Utc::now(),
            kind,
        }
    }
}
