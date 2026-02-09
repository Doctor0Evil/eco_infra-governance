use crate::{
    aln::{AlnShard, RoutingDecision, RoutingPolicy},
    audit::{event::AuditEvent, sink::AuditSink},
};

pub struct VscArtemisEnclave;

impl VscArtemisEnclave {
    pub fn process_shard<P: AuditSink>(
        shard: AlnShard,
        audit_sink: &mut P,
    ) -> Result<(), &'static str> {
        // Enforce routing policy before any optimization logic runs. [file:2][file:3]
        match RoutingPolicy::evaluate(&shard, audit_sink) {
            RoutingDecision::Allowed => {
                audit_sink.record(AuditEvent::routing_allowed("enclave_accept"));
                // Here you call your optimization / eco-routing solvers.
                Ok(())
            }
            RoutingDecision::Denied(reason) => Err(reason),
        }
    }
}
