use crate::aln_shard::AlnShard;
use crate::nanopolygon::{ImpactRating, NanopolygonSafetyObject};
use crate::governance_hooks::{GovernanceHooks, HitlTicket};

pub const POWER_THRESHOLD: f64 = 0.92;

pub enum RouteDecision {
    Execute,
    QueueHitl(HitlTicket),
    Reject(&'static str),
}

pub struct RoutingContext<'a, H: GovernanceHooks> {
    pub hooks: &'a H,
}

impl<'a, H: GovernanceHooks> RoutingContext<'a, H> {
    pub fn decide(
        &self,
        shard: &AlnShard,
        model_confidence: f64,
    ) -> RouteDecision {
        let np: &NanopolygonSafetyObject = &shard.payload;

        // Example: equal power-threshold + impact gating.
        let is_high_impact = matches!(np.impact_rating, ImpactRating::High(_) | ImpactRating::Critical(_));
        let is_manage = np.ai_rmf_function == "MANAGE" || np.ai_rmf_function == "GOVERN";

        if is_high_impact && model_confidence >= POWER_THRESHOLD {
            // Route to HITL queue, not direct execution.
            let ticket = self.hooks.enqueue_hitl(shard.clone(), model_confidence);
            return RouteDecision::QueueHitl(ticket);
        }

        if np.governance_meta.hitl_required && is_high_impact {
            let ticket = self.hooks.enqueue_hitl(shard.clone(), model_confidence);
            return RouteDecision::QueueHitl(ticket);
        }

        RouteDecision::Execute
    }
}
