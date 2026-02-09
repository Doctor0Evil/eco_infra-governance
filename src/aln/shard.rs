use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::nanopolygon::ValidatedNanopolygon;
use super::device_class::AlnDeviceClass;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnShardHeader {
    pub shard_id: Uuid,
    pub source_device_class: AlnDeviceClass,
    pub target_device_class: AlnDeviceClass,
    pub segment_label: String, // e.g., "eco-infra-routing".
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlnShard {
    pub header: AlnShardHeader,
    pub nanopolygon: ValidatedNanopolygon,
}
