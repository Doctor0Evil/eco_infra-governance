use crate::audit::{event::AuditEvent, sink::AuditSink};
use super::{AlnShard, device_class::AlnDeviceClass};

#[derive(Debug)]
pub enum RoutingDecision {
    Allowed,
    Denied(&'static str),
}

pub struct RoutingPolicy;

impl RoutingPolicy {
    pub fn evaluate<P: AuditSink>(shard: &AlnShard, audit_sink: &mut P) -> RoutingDecision {
        let src = shard.header.source_device_class;
        let dst = shard.header.target_device_class;

        // No cybernetic / BCI / neuromorphic classes exist here by construction. [file:3][file:5]
        if src.is_cybernetic() || dst.is_cybernetic() {
            audit_sink.record(AuditEvent::routing_denied("cybernetic_device"));
            return RoutingDecision::Denied("cybernetic devices are prohibited");
        }

        // Minimal example: disallow direct OT‑>IT lateral movement, enforce ALN segmentation. [file:1][file:2]
        match (src, dst) {
            (AlnDeviceClass::Sensor, AlnDeviceClass::DatacenterNode) => {
                audit_sink.record(AuditEvent::routing_allowed("sensor_to_dc"));
                RoutingDecision::Allowed
            }
            (AlnDeviceClass::ScadaGateway, AlnDeviceClass::SupercomputerNode) => {
                audit_sink.record(AuditEvent::routing_allowed("scada_to_supercomputer"));
                RoutingDecision::Allowed
            }
            _ => {
                audit_sink.record(AuditEvent::routing_denied("disallowed_path"));
                RoutingDecision::Denied("disallowed routing path under zero-trust ALN")
            }
        }
    }
}
