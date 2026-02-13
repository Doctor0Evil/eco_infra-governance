use chrono::Utc;
use geojson::Geometry;
use uuid::Uuid;

use ecoinfra_governance::nanopolygon::*;
use ecoinfra_governance::aln::*;

fn main() {
    let geometry = Geometry::new(geojson::Value::Polygon(vec![]));

    let geo = GeoIntelligence {
        location_band: "urban-core".to_string(),
        hazard_level: HazardLevel::Moderate,
        resource_stress: 0.7,
        infrastructure_criticality: 8,
    };

    let biospatial = BiospatialTelemetry {
        heat_stress: 0.3,
        pollution_exposure: 0.4,
    };

    let learning = LearningSignal {
        gradient_weight: 0.9,
    };

    let intelligence = IntelligenceIndex {
        quantified_safety_index: -0.2,
    };

    let species_rights = SpeciesRightsProfile {
        primary_species: SpeciesClass::CyberneticallyEnhancedHuman,
        recognizes_cybernetic_personhood: true,
        territory_protection: TerritoryProtectionLevel::SanctuaryCorridor,
        anti_discrimination_hard_floor: true,
    };

    let neurorights = NeurorightsProfile {
        allows_direct_neural_interfaces: false,
        neurorights_sanctuary: true,
        consent_requirement: ConsentRequirement::CommunityAndIndividual,
        appeal_path: AppealPathType::EcoInfraCouncil,
        hitl_trigger_threshold: 0.0,
    };

    let rights = RightsMetadata {
        species_rights,
        neurorights,
    };

    let np = NanopolygonSafetyObject {
        polygon_id: Uuid::new_v4(),
        geometry,
        geo,
        biospatial,
        learning,
        intelligence,
        rights,
        timestamp_utc: Utc::now(),
    };

    let header = AlnShardHeader {
        shard_id: Uuid::new_v4(),
        source_device_class: AlnDeviceClass::ScadaGateway,
        target_device_class: AlnDeviceClass::SupercomputerNode,
        segment_label: "eco-infra-routing".to_string(),
    };

    let shard = AlnShard { header, nanopolygon: np };
    let sanctuary_shard = SanctuaryRoutingShard::from_nanopolygon(shard);

    let enforce = is_sanctuary_enforced(
        &sanctuary_shard,
        sanctuary_shard.base.nanopolygon.intelligence.quantified_safety_index,
    );

    println!(
        "Sanctuary kind = {:?}, requires_explicit_consent = {}, enforce = {}",
        sanctuary_shard.kind,
        sanctuary_shard.requires_explicit_consent(),
        enforce,
    );
}
