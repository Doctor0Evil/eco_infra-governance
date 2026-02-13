use serde::{Deserialize, Serialize};

use crate::nanopolygon::{
    NanopolygonSafetyObject,
    TerritoryProtectionLevel,
    ConsentRequirement,
};

/// Classification of sanctuary‑relevant shards.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SanctuaryShardKind {
    /// A biophysical sanctuary corridor – transit allowed, no harmful ops.
    BiophysicalSanctuaryCorridor,
    /// A domain that requires explicit consent before high‑impact actions.
    ConsentRequiredCyberneticDomain,
    /// Not sanctuary‑relevant.
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctuaryRoutingShard {
    pub base: super::AlnShard,
    pub kind: SanctuaryShardKind,
}

impl SanctuaryRoutingShard {
    pub fn from_nanopolygon(base: super::AlnShard) -> Self {
        let np = &base.nanopolygon;

        let kind = match np.rights.species_rights.territory_protection {
            TerritoryProtectionLevel::SanctuaryCorridor => {
                SanctuaryShardKind::BiophysicalSanctuaryCorridor
            }
            _ => {
                match np.rights.neurorights.consent_requirement {
                    ConsentRequirement::CommunityMultisigRequired
                    | ConsentRequirement::IndividualConsentRequired
                    | ConsentRequirement::CommunityAndIndividual => {
                        SanctuaryShardKind::ConsentRequiredCyberneticDomain
                    }
                    ConsentRequirement::StandardGovernance => SanctuaryShardKind::None,
                }
            }
        };

        SanctuaryRoutingShard { base, kind }
    }

    /// Helper: does this shard require explicit consent before actuation?
    pub fn requires_explicit_consent(&self) -> bool {
        matches!(
            self.kind,
            SanctuaryShardKind::ConsentRequiredCyberneticDomain
        )
    }
}

/// Pure function you can plug into routing policies / HITL guards.
pub fn is_sanctuary_enforced(
    shard: &SanctuaryRoutingShard,
    safety_index: f64,
) -> bool {
    let neurorights = &shard.base.nanopolygon.rights.neurorights;

    // If neurorights sanctuary, always enforce HITL + strict routing.
    if neurorights.neurorights_sanctuary {
        return true;
    }

    // Else, enforce if HITL threshold is crossed.
    neurorights.requires_hitl(safety_index)
}
