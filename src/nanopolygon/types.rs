use chrono::{DateTime, Utc};
use geojson::Geometry;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::policy::governance_metadata::GovernanceMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HazardLevel {
    Low,
    Moderate,
    High,
    Extreme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoIntelligence {
    pub location_band: String,
    pub hazard_level: HazardLevel,
    pub resource_stress: f64,          // 0.0–1.0 normalized.
    pub infrastructure_criticality: u8 // 1–10.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiospatialTelemetry {
    pub heat_stress: f64,             // 0.0–1.0.
    pub pollution_exposure: f64,      // 0.0–1.0.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSignal {
    pub gradient_weight: f64,         // 0.0–1.0.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceIndex {
    pub quantified_safety_index: f64, // -1.0–1.0.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NanopolygonSafetyObject {
    pub polygon_id: Uuid,
    pub geometry: Geometry,
    pub geo: GeoIntelligence,
    pub biospatial: BiospatialTelemetry,
    pub learning: LearningSignal,
    pub intelligence: IntelligenceIndex,
    pub metadata: GovernanceMetadata,
    pub timestamp_utc: DateTime<Utc>,
}
