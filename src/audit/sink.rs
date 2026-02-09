use super::event::AuditEvent;

pub trait AuditSink {
    fn record(&mut self, event: AuditEvent);
}

// Example in-memory sink; production hooking goes to append-only, chained storage. [file:1][file:5]
pub struct InMemoryAuditSink {
    pub events: Vec<AuditEvent>,
}

impl InMemoryAuditSink {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
}

impl AuditSink for InMemoryAuditSink {
    fn record(&mut self, event: AuditEvent) {
        self.events.push(event);
    }
}
