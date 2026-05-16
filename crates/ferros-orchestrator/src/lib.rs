#![forbid(unsafe_code)]

mod packet;

pub use packet::{
    has_non_empty_evidence_refs, try_transition, validate_transition_requirements,
    GatekeeperDecision, MonitorPacket, PacketAuditEntry, PacketState,
    PacketTransitionApplied, PacketTransitionError, ReviewVerdict,
};