use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlnDeviceClass {
    Sensor,
    ScadaGateway,
    MicrogridController,
    DatacenterNode,
    SupercomputerNode,
}

impl AlnDeviceClass {
    pub fn is_cybernetic(&self) -> bool {
        false // By definition, this enum excludes BCI/cybernetic classes. [file:3][file:5]
    }
}
