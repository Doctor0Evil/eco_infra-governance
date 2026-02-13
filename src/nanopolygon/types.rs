use chrono::{DateTime, Utc};
use geojson::Geometry;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::nanopolygon::neurorights::{SpeciesRightsProfile, NeurorightsProfile};

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
    /// 0.0–1.0 normalized.
    pub resource_stress: f64,
    /// 1–10 criticality of local infrastructure.
    pub infrastructure_criticality: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiospatialTelemetry {
    /// 0.0–1.0; de‑identified heat stress index.
    pub heat_stress: f64,
    /// 0.0–1.0; de‑identified pollution exposure.
    pub pollution_exposure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSignal {
    /// 0.0–1.0; learning importance weight.
    pub gradient_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceIndex {
    /// −1.0 to 1.0; composite risk/resilience/equity.
    pub quantified_safety_index: f64,
}

/// New: explicit rights metadata for this nanopolygon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RightsMetadata {
    /// Species‑rights envelope for this polygon.
    pub species_rights: SpeciesRightsProfile,
    /// Neurorights and cognitive safety envelope.
    pub neurorights: NeurorightsProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NanopolygonSafetyObject {
    pub polygon_id: Uuid,
    pub geometry: Geometry,
    pub geo: GeoIntelligence,
    pub biospatial: BiospatialTelemetry,
    pub learning: LearningSignal,
    pub intelligence: IntelligenceIndex,
    /// Rights + neurorights overlays, versioned and auditable.
    pub rights: RightsMetadata,
    pub timestamp_utc: DateTime<Utc>,
}
