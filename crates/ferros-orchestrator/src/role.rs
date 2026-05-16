use crate::{
    GatekeeperDecision, PacketClaimRole, PacketRepository, PacketState, PacketTransitionError,
    PacketTransitionRequest, ReviewVerdict,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickReport {
    pub role: PacketClaimRole,
    pub claimed_packet_id: Option<String>,
    pub advanced_to: Option<PacketState>,
}

impl TickReport {
    fn idle(role: PacketClaimRole) -> Self {
        Self {
            role,
            claimed_packet_id: None,
            advanced_to: None,
        }
    }

    fn claimed(role: PacketClaimRole, packet_id: String) -> Self {
        Self {
            role,
            claimed_packet_id: Some(packet_id),
            advanced_to: None,
        }
    }

    fn advanced(role: PacketClaimRole, packet_id: String, advanced_to: PacketState) -> Self {
        Self {
            role,
            claimed_packet_id: Some(packet_id),
            advanced_to: Some(advanced_to),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoleAgentError {
    Transition(PacketTransitionError),
    Repository(String),
}

impl From<PacketTransitionError> for RoleAgentError {
    fn from(value: PacketTransitionError) -> Self {
        Self::Transition(value)
    }
}

pub trait RoleAgent {
    fn role(&self) -> PacketClaimRole;

    fn tick(&self, repo: &mut dyn PacketRepository, at: &str)
        -> Result<TickReport, RoleAgentError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StubManagerAgent;

impl RoleAgent for StubManagerAgent {
    fn role(&self) -> PacketClaimRole {
        PacketClaimRole::Manager
    }

    fn tick(
        &self,
        repo: &mut dyn PacketRepository,
        at: &str,
    ) -> Result<TickReport, RoleAgentError> {
        let Some(claim) = repo.claim_next(self.role()) else {
            return Ok(TickReport::idle(self.role()));
        };
        let packet_id = claim.packet_id;
        repo.apply_transition(PacketTransitionRequest {
            packet_id: packet_id.clone(),
            to_state: PacketState::InProgress,
            actor: "stub-manager-agent".to_owned(),
            reason: "manager claimed packet".to_owned(),
            at: at.to_owned(),
            evidence_refs: vec![],
        })?;
        Ok(TickReport::advanced(
            self.role(),
            packet_id,
            PacketState::InProgress,
        ))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StubWorkerAgent;

impl RoleAgent for StubWorkerAgent {
    fn role(&self) -> PacketClaimRole {
        PacketClaimRole::Worker
    }

    fn tick(
        &self,
        repo: &mut dyn PacketRepository,
        at: &str,
    ) -> Result<TickReport, RoleAgentError> {
        let Some(claim) = repo.claim_next(self.role()) else {
            return Ok(TickReport::idle(self.role()));
        };
        let packet_id = claim.packet_id;
        repo.apply_transition(PacketTransitionRequest {
            packet_id: packet_id.clone(),
            to_state: PacketState::AwaitingReview,
            actor: "stub-worker-agent".to_owned(),
            reason: "worker completed packet".to_owned(),
            at: at.to_owned(),
            evidence_refs: vec![format!("evidence://{packet_id}")],
        })?;
        Ok(TickReport::advanced(
            self.role(),
            packet_id,
            PacketState::AwaitingReview,
        ))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StubReviewerAgent;

impl RoleAgent for StubReviewerAgent {
    fn role(&self) -> PacketClaimRole {
        PacketClaimRole::Reviewer
    }

    fn tick(
        &self,
        repo: &mut dyn PacketRepository,
        at: &str,
    ) -> Result<TickReport, RoleAgentError> {
        let Some(claim) = repo.claim_next(self.role()) else {
            return Ok(TickReport::idle(self.role()));
        };
        let packet_id = claim.packet_id;
        repo.set_review_verdict(&packet_id, ReviewVerdict::Approved, at.to_owned())
            .map_err(RoleAgentError::Repository)?;
        repo.apply_transition(PacketTransitionRequest {
            packet_id: packet_id.clone(),
            to_state: PacketState::Reviewed,
            actor: "stub-reviewer-agent".to_owned(),
            reason: "reviewer approved packet".to_owned(),
            at: at.to_owned(),
            evidence_refs: vec![],
        })?;
        Ok(TickReport::advanced(
            self.role(),
            packet_id,
            PacketState::Reviewed,
        ))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StubRecoveryAgent;

impl RoleAgent for StubRecoveryAgent {
    fn role(&self) -> PacketClaimRole {
        PacketClaimRole::Recovery
    }

    fn tick(
        &self,
        repo: &mut dyn PacketRepository,
        at: &str,
    ) -> Result<TickReport, RoleAgentError> {
        let Some(claim) = repo.claim_next(self.role()) else {
            return Ok(TickReport::idle(self.role()));
        };
        let packet_id = claim.packet_id;
        let packet = repo.packet(&packet_id).ok_or_else(|| {
            RoleAgentError::Repository("claimed packet missing from repository".to_owned())
        })?;
        let (next_state, reason) = if packet.can_retry() {
            (
                PacketState::DispatchedToManager,
                format!(
                    "recovery requeued retryable packet (attempt {} of {})",
                    packet.retry_count + 1,
                    packet.retry_budget
                ),
            )
        } else if packet.last_failure_retryable {
            (
                PacketState::HumanInterventionRequired,
                format!(
                    "recovery exhausted retry budget after {} attempts",
                    packet.retry_count
                ),
            )
        } else {
            (
                PacketState::HumanInterventionRequired,
                "recovery escalated non-retryable failure".to_owned(),
            )
        };
        repo.apply_transition(PacketTransitionRequest {
            packet_id: packet_id.clone(),
            to_state: next_state.clone(),
            actor: "stub-recovery-agent".to_owned(),
            reason,
            at: at.to_owned(),
            evidence_refs: vec![],
        })?;
        Ok(TickReport::advanced(self.role(), packet_id, next_state))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StubGatekeeperAgent;

impl RoleAgent for StubGatekeeperAgent {
    fn role(&self) -> PacketClaimRole {
        PacketClaimRole::Gatekeeper
    }

    fn tick(
        &self,
        repo: &mut dyn PacketRepository,
        at: &str,
    ) -> Result<TickReport, RoleAgentError> {
        let Some(claim) = repo.claim_next(self.role()) else {
            return Ok(TickReport::idle(self.role()));
        };
        let packet_id = claim.packet_id;
        let evidence_refs = repo
            .packet(&packet_id)
            .map(packet_evidence_refs)
            .unwrap_or_default();
        repo.set_gatekeeper_decision(&packet_id, GatekeeperDecision::Close, at.to_owned())
            .map_err(RoleAgentError::Repository)?;
        repo.apply_transition(PacketTransitionRequest {
            packet_id: packet_id.clone(),
            to_state: PacketState::Resolved,
            actor: "stub-gatekeeper-agent".to_owned(),
            reason: "gatekeeper closed packet".to_owned(),
            at: at.to_owned(),
            evidence_refs,
        })?;
        Ok(TickReport::advanced(
            self.role(),
            packet_id,
            PacketState::Resolved,
        ))
    }
}

fn packet_evidence_refs(packet: &crate::MonitorPacket) -> Vec<String> {
    let mut evidence_refs = Vec::new();
    for entry in &packet.audit_trail {
        for reference in &entry.evidence_refs {
            if reference.trim().is_empty() || evidence_refs.contains(reference) {
                continue;
            }
            evidence_refs.push(reference.clone());
        }
    }
    evidence_refs
}

#[cfg(test)]
mod tests {
    use crate::{InMemoryPacketRepository, MonitorPacket, PacketRepository, PacketState};

    use super::{
        RoleAgent, StubGatekeeperAgent, StubManagerAgent, StubRecoveryAgent,
        StubReviewerAgent, StubWorkerAgent,
    };

    fn make_packet(id: &str, manager: &str, state: PacketState) -> MonitorPacket {
        MonitorPacket {
            id: id.to_owned(),
            session_id: "test-session".to_owned(),
            origin_message_id: None,
            parent_packet_id: None,
            work_order_id: None,
            manager: manager.to_owned(),
            state,
            review_verdict: None,
            gatekeeper_decision: None,
            lifecycle_thread_id: None,
            notification_id: None,
            created_at: "2026-01-01T00:00:00Z".to_owned(),
            updated_at: "2026-01-01T00:00:00Z".to_owned(),
            summary: "test packet".to_owned(),
            last_error: None,
            retry_count: 0,
            retry_budget: 0,
            last_failure_retryable: false,
            audit_seq: 0,
            audit_trail: vec![],
        }
    }

    #[test]
    fn stub_role_agents_drive_packet_to_resolved() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet(
            "flow-1",
            "Software Architect",
            PacketState::DispatchedToManager,
        ));

        let manager = StubManagerAgent;
        let worker = StubWorkerAgent;
        let reviewer = StubReviewerAgent;
        let gatekeeper = StubGatekeeperAgent;

        let manager_report = manager.tick(&mut repo, "2026-01-01T00:00:01Z").unwrap();
        let worker_report = worker.tick(&mut repo, "2026-01-01T00:00:02Z").unwrap();
        let reviewer_report = reviewer.tick(&mut repo, "2026-01-01T00:00:03Z").unwrap();
        let gatekeeper_report = gatekeeper.tick(&mut repo, "2026-01-01T00:00:04Z").unwrap();

        assert_eq!(manager_report.claimed_packet_id.as_deref(), Some("flow-1"));
        assert_eq!(worker_report.claimed_packet_id.as_deref(), Some("flow-1"));
        assert_eq!(reviewer_report.claimed_packet_id.as_deref(), Some("flow-1"));
        assert_eq!(
            gatekeeper_report.claimed_packet_id.as_deref(),
            Some("flow-1")
        );

        let packet = repo.packet("flow-1").unwrap();
        assert_eq!(packet.state, PacketState::Resolved);
        assert_eq!(packet.review_verdict, Some(crate::ReviewVerdict::Approved));
        assert_eq!(
            packet.gatekeeper_decision,
            Some(crate::GatekeeperDecision::Close)
        );
        assert_eq!(packet.audit_trail.len(), 4);
        assert!(!packet.audit_trail[1].evidence_refs.is_empty());
        assert!(!packet.audit_trail[3].evidence_refs.is_empty());
    }

    #[test]
    fn stub_role_agents_idle_when_no_matching_packet_exists() {
        let mut repo = InMemoryPacketRepository::default();

        let report = StubWorkerAgent
            .tick(&mut repo, "2026-01-01T00:00:01Z")
            .unwrap();

        assert!(report.claimed_packet_id.is_none());
        assert!(report.advanced_to.is_none());
    }

    #[test]
    fn stub_recovery_agent_requeues_retryable_failed_packet() {
        let mut repo = InMemoryPacketRepository::default();
        let mut packet = make_packet("failed-1", "Software Architect", PacketState::Failed);
        packet.retry_budget = 2;
        packet.last_failure_retryable = true;
        repo.register_packet(packet);

        let report = StubRecoveryAgent
            .tick(&mut repo, "2026-01-01T00:00:01Z")
            .unwrap();

        assert_eq!(report.claimed_packet_id.as_deref(), Some("failed-1"));
        assert_eq!(report.advanced_to, Some(PacketState::DispatchedToManager));
        let packet = repo.packet("failed-1").unwrap();
        assert_eq!(packet.state, PacketState::DispatchedToManager);
        assert_eq!(packet.retry_count, 1);
    }

    #[test]
    fn stub_recovery_agent_escalates_failed_packet_when_budget_is_exhausted() {
        let mut repo = InMemoryPacketRepository::default();
        let mut packet = make_packet("failed-2", "Software Architect", PacketState::Failed);
        packet.retry_budget = 1;
        packet.retry_count = 1;
        packet.last_failure_retryable = true;
        repo.register_packet(packet);

        let report = StubRecoveryAgent
            .tick(&mut repo, "2026-01-01T00:00:01Z")
            .unwrap();

        assert_eq!(report.claimed_packet_id.as_deref(), Some("failed-2"));
        assert_eq!(
            report.advanced_to,
            Some(PacketState::HumanInterventionRequired)
        );
        assert_eq!(
            repo.packet("failed-2").unwrap().state,
            PacketState::HumanInterventionRequired
        );
    }
}
