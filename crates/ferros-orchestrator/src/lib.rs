#![forbid(unsafe_code)]

mod loop_runner;
mod packet;
mod role;

pub use loop_runner::{OrchestratorLoop, OrchestratorMode};
pub use packet::{
    has_non_empty_evidence_refs, try_transition, validate_transition_requirements,
    GatekeeperDecision, InMemoryPacketRepository, MonitorPacket, PacketAuditEntry, PacketClaim,
    PacketClaimRole, PacketRepository, PacketState, PacketTransitionApplied, PacketTransitionError,
    PacketTransitionRequest, ReviewVerdict,
};
pub use role::{
    RoleAgent, RoleAgentError, StubGatekeeperAgent, StubManagerAgent, StubRecoveryAgent,
    StubReviewerAgent, StubWorkerAgent, TickReport,
};
