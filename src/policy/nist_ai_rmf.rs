use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct AIRmfFunction: u32 {
        const GOVERN = 0b0001;
        const MAP    = 0b0010;
        const MEASURE= 0b0100;
        const MANAGE = 0b1000;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRmfProfile {
    pub functions: AIRmfFunction,
    pub requires_human_in_loop: bool,
    pub requires_appeal_path: bool,
}
