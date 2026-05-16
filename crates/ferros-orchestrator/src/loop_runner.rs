use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    PacketRepository, RoleAgent, RoleAgentError, StubGatekeeperAgent, StubManagerAgent,
    StubRecoveryAgent, StubReviewerAgent, StubWorkerAgent, TickReport,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrchestratorMode {
    #[default]
    Disabled,
    Stub,
    Live,
}

impl fmt::Display for OrchestratorMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrchestratorMode::Disabled => formatter.write_str("disabled"),
            OrchestratorMode::Stub => formatter.write_str("stub"),
            OrchestratorMode::Live => formatter.write_str("live"),
        }
    }
}

pub struct OrchestratorLoop {
    agents: Vec<Box<dyn RoleAgent>>,
}

impl OrchestratorLoop {
    pub fn new(agents: Vec<Box<dyn RoleAgent>>) -> Self {
        Self { agents }
    }

    pub fn stub() -> Self {
        Self::new(vec![
            Box::new(StubRecoveryAgent),
            Box::new(StubGatekeeperAgent),
            Box::new(StubReviewerAgent),
            Box::new(StubWorkerAgent),
            Box::new(StubManagerAgent),
        ])
    }

    pub fn tick_once(
        &self,
        repo: &mut dyn PacketRepository,
        at: &str,
    ) -> Result<Vec<TickReport>, RoleAgentError> {
        repo.reclaim_expired(at)
            .map_err(RoleAgentError::Repository)?;
        let mut reports = Vec::with_capacity(self.agents.len());
        for agent in &self.agents {
            reports.push(agent.tick(repo, at)?);
        }
        Ok(reports)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        InMemoryPacketRepository, MonitorPacket, PacketRepository, PacketState,
    };

    use super::OrchestratorLoop;

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
            registration_idempotency_key: None,
            retry_count: 0,
            retry_budget: 0,
            last_failure_retryable: false,
            lease_role: None,
            lease_expires_at: None,
            audit_seq: 0,
            audit_trail: vec![],
        }
    }

    #[test]
    fn orchestrator_loop_advances_packet_across_ticks_in_priority_order() {
        let loop_runner = OrchestratorLoop::stub();
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet(
            "loop-1",
            "Software Architect",
            PacketState::DispatchedToManager,
        ))
        .expect("packet registration should succeed");

        let tick1 = loop_runner
            .tick_once(&mut repo, "2026-01-01T00:00:01Z")
            .unwrap();
        let tick2 = loop_runner
            .tick_once(&mut repo, "2026-01-01T00:00:02Z")
            .unwrap();
        let tick3 = loop_runner
            .tick_once(&mut repo, "2026-01-01T00:00:03Z")
            .unwrap();
        let tick4 = loop_runner
            .tick_once(&mut repo, "2026-01-01T00:00:04Z")
            .unwrap();

        assert_eq!(repo.packet("loop-1").unwrap().state, PacketState::Resolved);
        assert_eq!(
            tick1
                .iter()
                .filter(|report| report.claimed_packet_id.is_some())
                .count(),
            1
        );
        assert_eq!(
            tick2
                .iter()
                .filter(|report| report.claimed_packet_id.is_some())
                .count(),
            1
        );
        assert_eq!(
            tick3
                .iter()
                .filter(|report| report.claimed_packet_id.is_some())
                .count(),
            1
        );
        assert_eq!(
            tick4
                .iter()
                .filter(|report| report.claimed_packet_id.is_some())
                .count(),
            1
        );
        assert_eq!(tick1[4].advanced_to, Some(PacketState::InProgress));
        assert_eq!(tick2[3].advanced_to, Some(PacketState::AwaitingReview));
        assert_eq!(tick3[2].advanced_to, Some(PacketState::Reviewed));
        assert_eq!(tick4[1].advanced_to, Some(PacketState::Resolved));
    }
}
