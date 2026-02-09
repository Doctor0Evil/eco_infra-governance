use super::types::{NanopolygonSafetyObject, GeoIntelligence, BiospatialTelemetry, LearningSignal, IntelligenceIndex};
use crate::audit::event::AuditEvent;
use crate::audit::sink::AuditSink;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("numeric field out of allowed range")]
    RangeError,
    #[error("governance invariants not satisfied: {0}")]
    Governance(&'static str),
}

pub struct ValidatedNanopolygon(NanopolygonSafetyObject);

impl ValidatedNanopolygon {
    pub fn inner(&self) -> &NanopolygonSafetyObject {
        &self.0
    }
}

// Basic numeric range enforcement + governance invariants.
pub fn validate<P: AuditSink>(
    np: NanopolygonSafetyObject,
    audit_sink: &mut P,
) -> Result<ValidatedNanopolygon, ValidationError> {
    fn in_01(v: f64) -> bool {
        (0.0..=1.0).contains(&v)
    }

    let GeoIntelligence {
        resource_stress,
        infrastructure_criticality,
        ..
    } = np.geo;

    let BiospatialTelemetry {
        heat_stress,
        pollution_exposure,
    } = np.biospatial;

    let LearningSignal { gradient_weight } = np.learning;
    let IntelligenceIndex {
        quantified_safety_index,
    } = np.intelligence;

    if !in_01(resource_stress)
        || !(1..=10).contains(&infrastructure_criticality)
        || !in_01(heat_stress)
        || !in_01(pollution_exposure)
        || !in_01(gradient_weight)
        || !(-1.0..=1.0).contains(&quantified_safety_index)
    {
        audit_sink.record(AuditEvent::validation_failed("range_violation"));
        return Err(ValidationError::RangeError);
    }

    if let Err(e) = np.assert_invariant_governance() {
        audit_sink.record(AuditEvent::validation_failed("governance_invariant_violation"));
        return Err(ValidationError::Governance(e));
    }

    audit_sink.record(AuditEvent::validation_success());
    Ok(ValidatedNanopolygon(np))
}
