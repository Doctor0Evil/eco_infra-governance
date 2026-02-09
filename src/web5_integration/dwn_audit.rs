use crate::governance_hooks::{AppealDecision, HitlTicket};
use crate::aln_shard::AlnShard;

pub trait DwnAudit {
    fn append_hitl_event(&self, ticket: &HitlTicket, shard: &AlnShard, confidence: f64);
    fn append_appeal_event(
        &self,
        ticket: &HitlTicket,
        human_did: &str,
        decision: &AppealDecision,
        reason: &str,
    );
}
