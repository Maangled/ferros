#![forbid(unsafe_code)]

mod loop_runner;
mod packet;
mod role;

pub use loop_runner::{OrchestratorLoop, OrchestratorMode};
pub use packet::{
    has_non_empty_evidence_refs, try_transition, validate_transition_requirements,
    FilePacketRepository, GatekeeperDecision, InMemoryPacketRepository, MonitorPacket,
    PacketAuditEntry, PacketAuditKind, PacketClaim, PacketClaimRole,
    PacketEnqueueRequest, PacketEnqueueResult, PacketEvidenceAppendRequest,
    PacketEvidenceAppendResult, PacketLifecycleOutcome, PacketRepository,
    PacketRepositorySnapshot, PacketState, PacketTransitionApplied, PacketTransitionError,
    PacketTransitionRequest, ReviewVerdict,
};
pub use role::{
    RoleAgent, RoleAgentError, StubGatekeeperAgent, StubManagerAgent, StubRecoveryAgent,
    StubReviewerAgent, StubWorkerAgent, TickReport,
};
