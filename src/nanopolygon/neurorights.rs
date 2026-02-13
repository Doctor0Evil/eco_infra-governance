use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeciesClass {
    Human,
    NonHumanAnimal,
    SyntheticLifeform,
    CyberneticallyEnhancedHuman,
    CyberneticallyEnhancedNonHuman,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerritoryProtectionLevel {
    /// No special sanctuary status.
    None,
    /// Biophysical sanctuary: no extractive or harmful activity.
    Sanctuary,
    /// Transit corridor with strict non‑interference rules.
    SanctuaryCorridor,
    /// Experimental zone with heightened oversight.
    RegulatedExperimentZone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsentRequirement {
    /// Normal ALN governance; no extra consent.
    StandardGovernance,
    /// Explicit community‑level consent required (multisig / vote).
    CommunityMultisigRequired,
    /// Individual DID‑based consent required where applicable.
    IndividualConsentRequired,
    /// Both community and individual layers must approve.
    CommunityAndIndividual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppealPathType {
    /// Routed to Eco‑Infra Council / CAIO board.
    EcoInfraCouncil,
    /// Routed to independent ethics tribunal.
    EthicsPanel,
    /// Routed to local community governance body.
    CommunityCouncil,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesRightsProfile {
    pub primary_species: SpeciesClass,
    /// Are cybernetically enhanced beings explicitly recognized here?
    pub recognizes_cybernetic_personhood: bool,
    /// Territorial protection strength.
    pub territory_protection: TerritoryProtectionLevel,
    /// Disallow any operations that systematically disadvantage this class.
    pub anti_discrimination_hard_floor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurorightsProfile {
    /// Whether invasive or direct neural interfaces are allowed at all
    /// (control plane remains prohibited by virtanetv1, this is for
    /// observational / medical contexts only).
    pub allows_direct_neural_interfaces: bool,
    /// True if this polygon is a neurorights sanctuary (no cognitive
    /// experimentation, profiling, or covert manipulation).
    pub neurorights_sanctuary: bool,
    /// Consent model before any high‑impact action affecting this polygon.
    pub consent_requirement: ConsentRequirement,
    /// Where appeals are routed when rights are contested.
    pub appeal_path: AppealPathType,
    /// Human‑review threshold: if quantified_safety_index or similar metrics
    /// cross this value, HITL review is mandatory before actuation.
    pub hitl_trigger_threshold: f64,
}

impl NeurorightsProfile {
    /// Simple guard you can call from routing or policy code.
    pub fn requires_hitl(&self, safety_index: f64) -> bool {
        safety_index <= self.hitl_trigger_threshold
    }
}
