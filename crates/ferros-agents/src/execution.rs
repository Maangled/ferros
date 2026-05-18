use std::fmt;

use ferros_orchestrator::MonitorPacket;
use serde::{Deserialize, Serialize};

use crate::agent::Agent;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PacketExecutionRequest {
    pub packet: MonitorPacket,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PacketExecutionDisposition {
    Completed,
    RetryableFailure,
    PermanentFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PacketExecutionLifecycleOutcome {
    pub kind: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escalation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_agent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PacketExecutionReport {
    pub packet_id: String,
    pub summary: String,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    pub disposition: PacketExecutionDisposition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_outcome: Option<PacketExecutionLifecycleOutcome>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lifecycle_errors: Vec<String>,
}

pub trait PacketExecutableAgent: Agent {
    fn execute_packet(
        &mut self,
        request: &PacketExecutionRequest,
    ) -> Result<PacketExecutionReport, Self::Error>;
}

pub trait WorkerDriver {
    type Error;

    fn execute(
        &mut self,
        request: PacketExecutionRequest,
    ) -> Result<PacketExecutionReport, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InProcessWorkerDriverError<E> {
    Agent(E),
}

impl<E: fmt::Display> fmt::Display for InProcessWorkerDriverError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Agent(error) => write!(f, "packet execution failed: {error}"),
        }
    }
}

impl<E> std::error::Error for InProcessWorkerDriverError<E> where E: std::error::Error + 'static {}

#[derive(Debug, Clone)]
pub struct InProcessWorkerDriver<A> {
    agent: A,
}

impl<A> InProcessWorkerDriver<A> {
    pub fn new(agent: A) -> Self {
        Self { agent }
    }

    #[must_use]
    pub fn agent(&self) -> &A {
        &self.agent
    }

    pub fn agent_mut(&mut self) -> &mut A {
        &mut self.agent
    }

    pub fn into_inner(self) -> A {
        self.agent
    }
}

impl<A> WorkerDriver for InProcessWorkerDriver<A>
where
    A: PacketExecutableAgent,
{
    type Error = InProcessWorkerDriverError<A::Error>;

    fn execute(
        &mut self,
        request: PacketExecutionRequest,
    ) -> Result<PacketExecutionReport, Self::Error> {
        self.agent
            .execute_packet(&request)
            .map_err(InProcessWorkerDriverError::Agent)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use ferros_profile::ProfileId;

    use super::{
        InProcessWorkerDriver, InProcessWorkerDriverError, PacketExecutableAgent,
        PacketExecutionDisposition, PacketExecutionLifecycleOutcome, PacketExecutionReport,
        PacketExecutionRequest, WorkerDriver,
    };
    use crate::agent::{Agent, AgentStatus};
    use crate::manifest::{AgentName, CapabilityRequirement};
    use ferros_orchestrator::{MonitorPacket, PacketState};

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum StubPacketAgentError {
        NotRunning,
    }

    impl fmt::Display for StubPacketAgentError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::NotRunning => write!(f, "agent must be running"),
            }
        }
    }

    impl std::error::Error for StubPacketAgentError {}

    #[derive(Debug, Clone)]
    struct StubPacketAgent {
        id: AgentName,
        capabilities: Vec<CapabilityRequirement>,
        status: AgentStatus,
    }

    impl StubPacketAgent {
        fn new() -> Self {
            let profile_id = ProfileId::new("profile-alpha").expect("valid profile id");
            Self {
                id: AgentName::new("packet-stub").expect("valid agent name"),
                capabilities: vec![CapabilityRequirement::new(profile_id, "agent.packet.execute")],
                status: AgentStatus::Registered,
            }
        }
    }

    impl Agent for StubPacketAgent {
        type Error = StubPacketAgentError;

        fn id(&self) -> &AgentName {
            &self.id
        }

        fn capabilities(&self) -> &[CapabilityRequirement] {
            &self.capabilities
        }

        fn start(&mut self) -> Result<(), Self::Error> {
            self.status = AgentStatus::Running;
            Ok(())
        }

        fn stop(&mut self) -> Result<(), Self::Error> {
            self.status = AgentStatus::Stopped;
            Ok(())
        }

        fn status(&self) -> AgentStatus {
            self.status
        }
    }

    impl PacketExecutableAgent for StubPacketAgent {
        fn execute_packet(
            &mut self,
            request: &PacketExecutionRequest,
        ) -> Result<PacketExecutionReport, Self::Error> {
            if self.status != AgentStatus::Running {
                return Err(StubPacketAgentError::NotRunning);
            }

            Ok(PacketExecutionReport {
                packet_id: request.packet.id.clone(),
                summary: format!("executed {}", request.packet.summary),
                evidence_refs: vec![format!("worker://{}", request.packet.id)],
                disposition: PacketExecutionDisposition::Completed,
                lifecycle_outcome: None,
                lifecycle_errors: Vec::new(),
            })
        }
    }

    fn make_request(packet_id: &str) -> PacketExecutionRequest {
        PacketExecutionRequest {
            packet: MonitorPacket {
                id: packet_id.to_owned(),
                session_id: "session-1".to_owned(),
                origin_message_id: None,
                parent_packet_id: None,
                work_order_id: Some("WO-1".to_owned()),
                manager: "Software Architect".to_owned(),
                state: PacketState::InProgress,
                review_verdict: None,
                gatekeeper_decision: None,
                lifecycle_thread_id: Some("cycle-1".to_owned()),
                notification_id: None,
                created_at: "2026-01-01T00:00:00Z".to_owned(),
                updated_at: "2026-01-01T00:00:00Z".to_owned(),
                summary: "run worker step".to_owned(),
                last_error: None,
                registration_idempotency_key: None,
                retry_count: 0,
                retry_budget: 0,
                last_failure_retryable: false,
                lease_role: None,
                lease_expires_at: None,
                audit_seq: 0,
                audit_trail: vec![],
            },
        }
    }

    #[test]
    fn in_process_worker_driver_executes_packet_with_running_agent() {
        let mut driver = InProcessWorkerDriver::new(StubPacketAgent::new());
        driver
            .agent_mut()
            .start()
            .expect("stub packet agent should start");

        let report = driver
            .execute(make_request("pkt-1"))
            .expect("driver should execute packet");

        assert_eq!(report.packet_id, "pkt-1");
        assert_eq!(report.summary, "executed run worker step");
        assert_eq!(report.evidence_refs, vec!["worker://pkt-1".to_owned()]);
        assert_eq!(report.disposition, PacketExecutionDisposition::Completed);
        assert_eq!(report.lifecycle_outcome, None);
        assert!(report.lifecycle_errors.is_empty());
    }

    #[test]
    fn in_process_worker_driver_surfaces_agent_errors() {
        let mut driver = InProcessWorkerDriver::new(StubPacketAgent::new());

        let error = driver
            .execute(make_request("pkt-2"))
            .expect_err("driver should fail when agent is not running");

        assert_eq!(
            error,
            InProcessWorkerDriverError::Agent(StubPacketAgentError::NotRunning)
        );
    }

    #[test]
    fn packet_execution_contract_round_trips_through_json() {
        let report = PacketExecutionReport {
            packet_id: "pkt-3".to_owned(),
            summary: "completed worker action".to_owned(),
            evidence_refs: vec!["artifact://pkt-3".to_owned()],
            disposition: PacketExecutionDisposition::RetryableFailure,
            lifecycle_outcome: Some(PacketExecutionLifecycleOutcome {
                kind: "work_order".to_owned(),
                summary: "follow-up work order issued".to_owned(),
                work_order_id: Some("WO-3".to_owned()),
                escalation_id: None,
                target_agent_id: Some("ferros-core".to_owned()),
                stop_reason: None,
            }),
            lifecycle_errors: vec!["stop contract requires a summary".to_owned()],
        };

        let encoded = serde_json::to_string(&report).expect("report should serialize");
        let decoded: PacketExecutionReport =
            serde_json::from_str(&encoded).expect("report should deserialize");

        assert_eq!(decoded, report);
    }
}