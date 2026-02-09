use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicsProfile {
    pub human_primacy: bool,
    pub equal_power_thresholds: bool,
    pub appeal_paths_available: bool,
}
