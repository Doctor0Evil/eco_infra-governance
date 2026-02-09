use crate::policy::{
    ethics::EthicsProfile,
    governance_metadata::GovernanceMetadata,
    nist_ai_rmf::AIRmfFunction,
    nist_80053::Sp80053Family,
};
use super::types::NanopolygonSafetyObject;

pub trait HasAI_RMFFunction {}
pub trait HasNIST800_53Controls {}
pub trait HasEthicalGuardrails {}

impl HasAI_RMFFunction for NanopolygonSafetyObject {}
impl HasNIST800_53Controls for NanopolygonSafetyObject {}
impl HasEthicalGuardrails for NanopolygonSafetyObject {}

impl NanopolygonSafetyObject {
    pub fn governance_metadata(&self) -> &GovernanceMetadata {
        &self.metadata
    }

    pub fn ethics_profile(&self) -> &EthicsProfile {
        &self.metadata.ethics_profile
    }

    pub fn assert_invariant_governance(&self) -> Result<(), &'static str> {
        // NIST AI RMF must include GOVERN and MANAGE for critical eco‑infra. [file:1][file:5]
        let ai_f = self.metadata.ai_rmf_profile.functions;
        if !ai_f.contains(AIRmfFunction::GOVERN | AIRmfFunction::MANAGE) {
            return Err("NIST AI RMF GOVERN|MANAGE missing");
        }

        // 800‑53 controls must include AC, AU, and SC families. [file:1][file:5]
        let sp = self.metadata.sp80053_profile.families;
        if !sp.contains(Sp80053Family::AC | Sp80053Family::AU | Sp80053Family::SC) {
            return Err("SP 800‑53 AC|AU|SC families required");
        }

        let ethics = &self.metadata.ethics_profile;
        if !(ethics.human_primacy && ethics.equal_power_thresholds && ethics.appeal_paths_available)
        {
            return Err("Ethical guardrails not fully satisfied");
        }

        Ok(())
    }
}
