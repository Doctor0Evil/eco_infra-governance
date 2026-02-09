use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Sp80053Family: u32 {
        const AC = 0b0001; // Access Control
        const AU = 0b0010; // Audit and Accountability
        const SC = 0b0100; // System and Communications Protection
        const RA = 0b1000; // Risk Assessment
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sp80053Profile {
    pub families: Sp80053Family,
}
