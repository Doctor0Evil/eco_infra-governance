use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{nist_ai_rmf::AIRmfProfile, nist_80053::Sp80053Profile, ethics::EthicsProfile};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceMetadata {
    pub jurisdiction_code: String,     // e.g., "US-FED", "EU-EN", etc.
    pub ai_rmf_profile: AIRmfProfile,
    pub sp80053_profile: Sp80053Profile,
    pub ethics_profile: EthicsProfile,
    pub data_owner_did: String,       // Web5 DID for ownership / CAIO oversight. [file:1][file:3]
    pub created_by_agent_id: Uuid,
    pub last_modified_by_agent_id: Uuid,
}
