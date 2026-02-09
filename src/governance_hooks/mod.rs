use crate::aln_shard::AlnShard;

#[derive(Clone, Debug)]
pub struct HitlTicket {
    pub ticket_id: String,
    pub shard_did: String,
}

pub trait GovernanceHooks {
    fn enqueue_hitl(&self, shard: AlnShard, confidence: f64) -> HitlTicket;
    fn record_appeal(
        &self,
        ticket: &HitlTicket,
        human_did: &str,
        decision: AppealDecision,
        reason: &str,
    );
}

#[derive(Clone, Debug)]
pub enum AppealDecision {
    Approved,
    Rejected,
    Modified,
}
