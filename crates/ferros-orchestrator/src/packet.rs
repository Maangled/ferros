use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PacketState {
    Staged,
    DispatchedToManager,
    InProgress,
    AwaitingReview,
    Reviewed,
    Resolved,
    Failed,
    HumanInterventionRequired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewVerdict {
    Approved,
    ChangesRequested,
    EscalateHuman,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GatekeeperDecision {
    Close,
    KeepOpen,
    EscalateHuman,
}

impl fmt::Display for PacketState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketState::Staged => formatter.write_str("staged"),
            PacketState::DispatchedToManager => formatter.write_str("dispatched_to_manager"),
            PacketState::InProgress => formatter.write_str("in_progress"),
            PacketState::AwaitingReview => formatter.write_str("awaiting_review"),
            PacketState::Reviewed => formatter.write_str("reviewed"),
            PacketState::Resolved => formatter.write_str("resolved"),
            PacketState::Failed => formatter.write_str("failed"),
            PacketState::HumanInterventionRequired => {
                formatter.write_str("human_intervention_required")
            }
            PacketState::Cancelled => formatter.write_str("cancelled"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketTransitionError {
    pub from: PacketState,
    pub to: PacketState,
    pub message: String,
}

impl fmt::Display for PacketTransitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cannot transition {} → {}: {}",
            self.from, self.to, self.message
        )
    }
}

impl std::error::Error for PacketTransitionError {}

pub fn try_transition(
    from: &PacketState,
    to: PacketState,
    actor: &str,
    reason: &str,
    _at: &str,
) -> Result<PacketState, PacketTransitionError> {
    if actor.is_empty() {
        return Err(PacketTransitionError {
            from: from.clone(),
            to,
            message: "actor must not be empty".to_owned(),
        });
    }
    if reason.is_empty() {
        return Err(PacketTransitionError {
            from: from.clone(),
            to,
            message: "reason must not be empty".to_owned(),
        });
    }
    let legal = matches!(
        (from, &to),
        (PacketState::Staged, PacketState::DispatchedToManager)
            | (PacketState::Staged, PacketState::HumanInterventionRequired)
            | (PacketState::Staged, PacketState::Failed)
            | (PacketState::DispatchedToManager, PacketState::InProgress)
            | (
                PacketState::DispatchedToManager,
                PacketState::HumanInterventionRequired
            )
            | (PacketState::InProgress, PacketState::AwaitingReview)
            | (PacketState::InProgress, PacketState::Failed)
            | (
                PacketState::InProgress,
                PacketState::HumanInterventionRequired
            )
            | (PacketState::AwaitingReview, PacketState::Reviewed)
            | (
                PacketState::AwaitingReview,
                PacketState::HumanInterventionRequired
            )
            | (PacketState::Reviewed, PacketState::Resolved)
            | (PacketState::Reviewed, PacketState::Failed)
            | (
                PacketState::Reviewed,
                PacketState::HumanInterventionRequired
            )
            | (PacketState::Failed, PacketState::DispatchedToManager)
            | (PacketState::Failed, PacketState::HumanInterventionRequired)
    );
    if legal {
        Ok(to)
    } else {
        Err(PacketTransitionError {
            from: from.clone(),
            to,
            message: "no legal edge from this state".to_owned(),
        })
    }
}

pub fn has_non_empty_evidence_refs(evidence_refs: &[String]) -> bool {
    evidence_refs
        .iter()
        .any(|reference| !reference.trim().is_empty())
}

pub fn validate_transition_requirements(
    from: &PacketState,
    to: PacketState,
    review_verdict: Option<&ReviewVerdict>,
    gatekeeper_decision: Option<&GatekeeperDecision>,
    evidence_refs: &[String],
) -> Result<(), PacketTransitionError> {
    if to == PacketState::AwaitingReview && !has_non_empty_evidence_refs(evidence_refs) {
        return Err(PacketTransitionError {
            from: from.clone(),
            to,
            message: "review-ready transition requires packet evidence".to_owned(),
        });
    }

    if to == PacketState::Reviewed && review_verdict.is_none() {
        return Err(PacketTransitionError {
            from: from.clone(),
            to,
            message: "reviewed transition requires reviewer verdict".to_owned(),
        });
    }

    if to == PacketState::Resolved {
        if !has_non_empty_evidence_refs(evidence_refs) {
            return Err(PacketTransitionError {
                from: from.clone(),
                to,
                message: "resolved transition requires packet evidence".to_owned(),
            });
        }
        if review_verdict != Some(&ReviewVerdict::Approved) {
            return Err(PacketTransitionError {
                from: from.clone(),
                to,
                message: "resolved transition requires approved reviewer verdict".to_owned(),
            });
        }
        if gatekeeper_decision != Some(&GatekeeperDecision::Close) {
            return Err(PacketTransitionError {
                from: from.clone(),
                to,
                message: "resolved transition requires gatekeeper close decision".to_owned(),
            });
        }
    }

    if from == &PacketState::Reviewed && to == PacketState::Failed {
        if !matches!(
            gatekeeper_decision,
            Some(GatekeeperDecision::KeepOpen) | Some(GatekeeperDecision::EscalateHuman)
        ) {
            return Err(PacketTransitionError {
                from: from.clone(),
                to,
                message: "failed transition from reviewed requires gatekeeper decision".to_owned(),
            });
        }
    }

    if from == &PacketState::Reviewed
        && to == PacketState::HumanInterventionRequired
        && gatekeeper_decision != Some(&GatekeeperDecision::EscalateHuman)
    {
        return Err(PacketTransitionError {
            from: from.clone(),
            to,
            message: "human intervention transition from reviewed requires escalate_human gatekeeper decision".to_owned(),
        });
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PacketAuditEntry {
    pub seq: usize,
    pub from: PacketState,
    pub to: PacketState,
    pub actor: String,
    pub reason: String,
    pub at: String,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketTransitionApplied {
    pub packet_id: String,
    pub from: PacketState,
    pub to: PacketState,
    pub seq: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MonitorPacket {
    pub id: String,
    pub session_id: String,
    #[serde(default)]
    pub origin_message_id: Option<String>,
    #[serde(default)]
    pub parent_packet_id: Option<String>,
    pub work_order_id: Option<String>,
    pub manager: String,
    pub state: PacketState,
    #[serde(default)]
    pub review_verdict: Option<ReviewVerdict>,
    #[serde(default)]
    pub gatekeeper_decision: Option<GatekeeperDecision>,
    pub lifecycle_thread_id: Option<String>,
    pub notification_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub summary: String,
    pub last_error: Option<String>,
    #[serde(default)]
    pub retry_count: usize,
    #[serde(default)]
    pub retry_budget: usize,
    #[serde(default)]
    pub last_failure_retryable: bool,
    #[serde(default)]
    pub audit_seq: usize,
    #[serde(default)]
    pub audit_trail: Vec<PacketAuditEntry>,
}

impl MonitorPacket {
    pub fn can_retry(&self) -> bool {
        self.last_failure_retryable && self.retry_count < self.retry_budget
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketTransitionRequest {
    pub packet_id: String,
    pub to_state: PacketState,
    pub actor: String,
    pub reason: String,
    pub at: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketClaimRole {
    Manager,
    Worker,
    Reviewer,
    Gatekeeper,
    Recovery,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketClaim {
    pub packet_id: String,
    pub role: PacketClaimRole,
    pub state: PacketState,
}

fn is_manager_role(manager: &str) -> bool {
    matches!(
        manager,
        "Software Architect"
            | "Business Agent"
            | "FERROS Agent Architect Agent"
            | "Coding Agent Architect"
            | "Business Agent Architect"
    )
}

fn packet_is_claimable_by_role(packet: &MonitorPacket, role: PacketClaimRole) -> bool {
    match role {
        PacketClaimRole::Manager => {
            packet.state == PacketState::DispatchedToManager && is_manager_role(&packet.manager)
        }
        PacketClaimRole::Worker => packet.state == PacketState::InProgress,
        PacketClaimRole::Reviewer => packet.state == PacketState::AwaitingReview,
        PacketClaimRole::Gatekeeper => packet.state == PacketState::Reviewed,
        PacketClaimRole::Recovery => packet.state == PacketState::Failed,
    }
}

pub trait PacketRepository {
    fn register_packet(&mut self, packet: MonitorPacket);
    fn packet(&self, packet_id: &str) -> Option<&MonitorPacket>;
    fn claim_next(&self, role: PacketClaimRole) -> Option<PacketClaim>;
    fn has_child_packets(&self, packet_id: &str) -> bool;
    fn apply_transition(
        &mut self,
        transition: PacketTransitionRequest,
    ) -> Result<Option<PacketTransitionApplied>, PacketTransitionError>;
    fn set_review_verdict(
        &mut self,
        packet_id: &str,
        verdict: ReviewVerdict,
        at: String,
    ) -> Result<bool, String>;
    fn set_gatekeeper_decision(
        &mut self,
        packet_id: &str,
        decision: GatekeeperDecision,
        at: String,
    ) -> Result<bool, String>;
    fn set_retry_policy(
        &mut self,
        packet_id: &str,
        retryable: Option<bool>,
        retry_budget: Option<usize>,
        at: String,
    ) -> Result<bool, String>;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct InMemoryPacketRepository {
    packets: Vec<MonitorPacket>,
}

impl InMemoryPacketRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> Iter<'_, MonitorPacket> {
        self.packets.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, MonitorPacket> {
        self.packets.iter_mut()
    }

    pub fn first(&self) -> Option<&MonitorPacket> {
        self.packets.first()
    }

    pub fn first_mut(&mut self) -> Option<&mut MonitorPacket> {
        self.packets.first_mut()
    }

    pub fn push(&mut self, packet: MonitorPacket) {
        self.register_packet(packet);
    }

    pub fn len(&self) -> usize {
        self.packets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }
}

impl Index<usize> for InMemoryPacketRepository {
    type Output = MonitorPacket;

    fn index(&self, index: usize) -> &Self::Output {
        &self.packets[index]
    }
}

impl IndexMut<usize> for InMemoryPacketRepository {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.packets[index]
    }
}

impl<'a> IntoIterator for &'a InMemoryPacketRepository {
    type Item = &'a MonitorPacket;
    type IntoIter = Iter<'a, MonitorPacket>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut InMemoryPacketRepository {
    type Item = &'a mut MonitorPacket;
    type IntoIter = IterMut<'a, MonitorPacket>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl PacketRepository for InMemoryPacketRepository {
    fn register_packet(&mut self, packet: MonitorPacket) {
        self.packets.push(packet);
    }

    fn packet(&self, packet_id: &str) -> Option<&MonitorPacket> {
        self.packets.iter().find(|packet| packet.id == packet_id)
    }

    fn claim_next(&self, role: PacketClaimRole) -> Option<PacketClaim> {
        self.packets
            .iter()
            .find(|packet| packet_is_claimable_by_role(packet, role))
            .map(|packet| PacketClaim {
                packet_id: packet.id.clone(),
                role,
                state: packet.state.clone(),
            })
    }

    fn has_child_packets(&self, packet_id: &str) -> bool {
        self.packets
            .iter()
            .any(|packet| packet.parent_packet_id.as_deref() == Some(packet_id))
    }

    fn apply_transition(
        &mut self,
        transition: PacketTransitionRequest,
    ) -> Result<Option<PacketTransitionApplied>, PacketTransitionError> {
        let PacketTransitionRequest {
            packet_id,
            to_state,
            actor,
            reason,
            at,
            evidence_refs,
        } = transition;

        let (from, seq, review_verdict, gatekeeper_decision) = {
            let Some(packet) = self.packet(&packet_id) else {
                return Ok(None);
            };
            (
                packet.state.clone(),
                packet.audit_seq + 1,
                packet.review_verdict.clone(),
                packet.gatekeeper_decision.clone(),
            )
        };

        validate_transition_requirements(
            &from,
            to_state.clone(),
            review_verdict.as_ref(),
            gatekeeper_decision.as_ref(),
            &evidence_refs,
        )?;

        let next = try_transition(&from, to_state, &actor, &reason, &at)?;
        let audit_entry = PacketAuditEntry {
            seq,
            from: from.clone(),
            to: next.clone(),
            actor,
            reason,
            at: at.clone(),
            evidence_refs,
        };
        let packet = self
            .packets
            .iter_mut()
            .find(|packet| packet.id == packet_id)
            .unwrap();
        packet.state = next.clone();
        packet.updated_at = at;
        if next == PacketState::Failed {
            packet.last_error = Some(audit_entry.reason.clone());
        }
        if from == PacketState::Failed && next == PacketState::DispatchedToManager {
            packet.retry_count += 1;
            packet.last_error = None;
            packet.review_verdict = None;
            packet.gatekeeper_decision = None;
        }
        packet.audit_seq = seq;
        packet.audit_trail.push(audit_entry);

        Ok(Some(PacketTransitionApplied {
            packet_id,
            from,
            to: next,
            seq,
        }))
    }

    fn set_review_verdict(
        &mut self,
        packet_id: &str,
        verdict: ReviewVerdict,
        at: String,
    ) -> Result<bool, String> {
        let Some(packet) = self
            .packets
            .iter_mut()
            .find(|packet| packet.id == packet_id)
        else {
            return Ok(false);
        };
        if packet.state != PacketState::AwaitingReview {
            return Err(
                "review verdict can only be set while packet is awaiting_review".to_owned(),
            );
        }
        packet.review_verdict = Some(verdict);
        packet.updated_at = at;
        Ok(true)
    }

    fn set_gatekeeper_decision(
        &mut self,
        packet_id: &str,
        decision: GatekeeperDecision,
        at: String,
    ) -> Result<bool, String> {
        let Some(packet) = self
            .packets
            .iter_mut()
            .find(|packet| packet.id == packet_id)
        else {
            return Ok(false);
        };
        if packet.state != PacketState::Reviewed {
            return Err("gatekeeper decision can only be set while packet is reviewed".to_owned());
        }
        packet.gatekeeper_decision = Some(decision);
        packet.updated_at = at;
        Ok(true)
    }

    fn set_retry_policy(
        &mut self,
        packet_id: &str,
        retryable: Option<bool>,
        retry_budget: Option<usize>,
        at: String,
    ) -> Result<bool, String> {
        let Some(packet) = self
            .packets
            .iter_mut()
            .find(|packet| packet.id == packet_id)
        else {
            return Ok(false);
        };

        if let Some(retryable) = retryable {
            packet.last_failure_retryable = retryable;
        }
        if let Some(retry_budget) = retry_budget {
            packet.retry_budget = retry_budget;
        }
        packet.updated_at = at;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        try_transition, validate_transition_requirements, GatekeeperDecision,
        InMemoryPacketRepository, MonitorPacket, PacketClaimRole, PacketRepository, PacketState,
        PacketTransitionRequest, ReviewVerdict,
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

    fn make_staged_packet(id: &str) -> MonitorPacket {
        make_packet(id, "test-manager", PacketState::Staged)
    }

    #[test]
    fn packet_state_serializes_current_wire_names() {
        fn wire(state: &PacketState) -> String {
            serde_json::to_string(state).expect("PacketState should serialize")
        }

        assert_eq!(
            wire(&PacketState::DispatchedToManager),
            "\"dispatched_to_manager\""
        );
        assert_eq!(
            wire(&PacketState::HumanInterventionRequired),
            "\"human_intervention_required\""
        );
        assert_eq!(wire(&PacketState::Staged), "\"staged\"");
        assert_eq!(wire(&PacketState::InProgress), "\"in_progress\"");
        assert_eq!(wire(&PacketState::AwaitingReview), "\"awaiting_review\"");
        assert_eq!(wire(&PacketState::Reviewed), "\"reviewed\"");
        assert_eq!(wire(&PacketState::Resolved), "\"resolved\"");
        assert_eq!(wire(&PacketState::Failed), "\"failed\"");
        assert_eq!(wire(&PacketState::Cancelled), "\"cancelled\"");
    }

    #[test]
    fn packet_transition_requires_actor_and_reason() {
        let empty_actor = try_transition(
            &PacketState::Staged,
            PacketState::DispatchedToManager,
            "",
            "valid reason",
            "2026-01-01T00:00:00Z",
        );
        assert!(empty_actor.is_err(), "empty actor must be rejected");

        let empty_reason = try_transition(
            &PacketState::Staged,
            PacketState::DispatchedToManager,
            "valid-actor",
            "",
            "2026-01-01T00:00:00Z",
        );
        assert!(empty_reason.is_err(), "empty reason must be rejected");
    }

    #[test]
    fn packet_transition_matrix_accepts_only_legal_edges() {
        let check = |from: &PacketState, to: PacketState| {
            try_transition(
                from,
                to,
                "test-actor",
                "test reason",
                "2026-01-01T00:00:00Z",
            )
        };

        assert!(check(&PacketState::Staged, PacketState::DispatchedToManager).is_ok());
        assert!(check(&PacketState::Staged, PacketState::HumanInterventionRequired).is_ok());
        assert!(check(&PacketState::Staged, PacketState::Failed).is_ok());
        assert!(check(&PacketState::DispatchedToManager, PacketState::InProgress).is_ok());
        assert!(check(
            &PacketState::DispatchedToManager,
            PacketState::HumanInterventionRequired
        )
        .is_ok());
        assert!(check(&PacketState::InProgress, PacketState::AwaitingReview).is_ok());
        assert!(check(&PacketState::InProgress, PacketState::Failed).is_ok());
        assert!(check(
            &PacketState::InProgress,
            PacketState::HumanInterventionRequired
        )
        .is_ok());
        assert!(check(&PacketState::AwaitingReview, PacketState::Reviewed).is_ok());
        assert!(check(
            &PacketState::AwaitingReview,
            PacketState::HumanInterventionRequired
        )
        .is_ok());
        assert!(check(&PacketState::Reviewed, PacketState::Resolved).is_ok());
        assert!(check(&PacketState::Reviewed, PacketState::Failed).is_ok());
        assert!(check(
            &PacketState::Reviewed,
            PacketState::HumanInterventionRequired
        )
        .is_ok());
        assert!(check(&PacketState::Failed, PacketState::DispatchedToManager).is_ok());
        assert!(check(&PacketState::Failed, PacketState::HumanInterventionRequired).is_ok());

        assert!(check(&PacketState::Resolved, PacketState::Staged).is_err());
        assert!(check(&PacketState::Failed, PacketState::Staged).is_err());
        assert!(check(&PacketState::HumanInterventionRequired, PacketState::Staged).is_err());
        assert!(check(&PacketState::Cancelled, PacketState::Staged).is_err());
    }

    #[test]
    fn validate_transition_requirements_rejects_resolved_without_gatekeeper_close() {
        let result = validate_transition_requirements(
            &PacketState::Reviewed,
            PacketState::Resolved,
            Some(&ReviewVerdict::Approved),
            Some(&GatekeeperDecision::KeepOpen),
            &["evidence://packet-1".to_owned()],
        );

        assert!(result.is_err(), "resolved requires gatekeeper close");
    }

    #[test]
    fn validate_transition_requirements_accepts_resolved_with_full_contract() {
        let result = validate_transition_requirements(
            &PacketState::Reviewed,
            PacketState::Resolved,
            Some(&ReviewVerdict::Approved),
            Some(&GatekeeperDecision::Close),
            &["evidence://packet-2".to_owned()],
        );

        assert!(result.is_ok(), "resolved should accept full contract");
    }

    #[test]
    fn in_memory_repository_apply_transition_updates_state_and_audit_trail() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_staged_packet("repo-p1"));

        let result = repo.apply_transition(PacketTransitionRequest {
            packet_id: "repo-p1".to_owned(),
            to_state: PacketState::DispatchedToManager,
            actor: "test-actor".to_owned(),
            reason: "test reason".to_owned(),
            at: "2026-01-01T00:00:01Z".to_owned(),
            evidence_refs: vec![],
        });

        assert!(result.is_ok(), "valid transition must succeed");
        let applied = result.unwrap().expect("packet should exist");
        assert_eq!(applied.from, PacketState::Staged);
        assert_eq!(applied.to, PacketState::DispatchedToManager);
        assert_eq!(applied.seq, 1);

        let packet = repo.packet("repo-p1").unwrap();
        assert_eq!(packet.state, PacketState::DispatchedToManager);
        assert_eq!(packet.audit_seq, 1);
        assert_eq!(packet.audit_trail.len(), 1);
    }

    #[test]
    fn in_memory_repository_rejects_review_verdict_outside_awaiting_review() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_staged_packet("repo-p2"));

        let result = repo.set_review_verdict(
            "repo-p2",
            ReviewVerdict::Approved,
            "2026-01-01T00:00:01Z".to_owned(),
        );

        assert!(result.is_err(), "verdict should require awaiting_review");
    }

    #[test]
    fn in_memory_repository_requeues_failed_packet_and_increments_retry_count() {
        let mut repo = InMemoryPacketRepository::default();
        let mut packet = make_packet("failed-p1", "Software Architect", PacketState::Failed);
        packet.retry_budget = 2;
        packet.last_failure_retryable = true;
        packet.review_verdict = Some(ReviewVerdict::Approved);
        packet.gatekeeper_decision = Some(GatekeeperDecision::KeepOpen);
        packet.last_error = Some("transient failure".to_owned());
        repo.register_packet(packet);

        let result = repo.apply_transition(PacketTransitionRequest {
            packet_id: "failed-p1".to_owned(),
            to_state: PacketState::DispatchedToManager,
            actor: "stub-recovery-agent".to_owned(),
            reason: "recovery requeued retryable packet".to_owned(),
            at: "2026-01-01T00:00:01Z".to_owned(),
            evidence_refs: vec![],
        });

        assert!(result.is_ok(), "requeue transition must succeed");
        let packet = repo.packet("failed-p1").unwrap();
        assert_eq!(packet.state, PacketState::DispatchedToManager);
        assert_eq!(packet.retry_count, 1);
        assert_eq!(packet.retry_budget, 2);
        assert!(packet.last_error.is_none());
        assert!(packet.review_verdict.is_none());
        assert!(packet.gatekeeper_decision.is_none());
    }

    #[test]
    fn in_memory_repository_sets_retry_policy_fields() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet(
            "failed-p2",
            "Software Architect",
            PacketState::Failed,
        ));

        let updated = repo
            .set_retry_policy(
                "failed-p2",
                Some(true),
                Some(3),
                "2026-01-01T00:00:01Z".to_owned(),
            )
            .unwrap();

        assert!(updated, "retry policy update should succeed");
        let packet = repo.packet("failed-p2").unwrap();
        assert!(packet.last_failure_retryable);
        assert_eq!(packet.retry_budget, 3);
    }

    #[test]
    fn claim_next_manager_returns_first_dispatched_manager_packet() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet(
            "not-manager",
            "test-manager",
            PacketState::DispatchedToManager,
        ));
        repo.register_packet(make_packet(
            "manager-1",
            "Software Architect",
            PacketState::DispatchedToManager,
        ));
        repo.register_packet(make_packet(
            "manager-2",
            "Business Agent Architect",
            PacketState::DispatchedToManager,
        ));

        let claim = repo
            .claim_next(PacketClaimRole::Manager)
            .expect("manager claim should exist");

        assert_eq!(claim.packet_id, "manager-1");
        assert_eq!(claim.role, PacketClaimRole::Manager);
        assert_eq!(claim.state, PacketState::DispatchedToManager);
    }

    #[test]
    fn claim_next_routes_other_roles_by_packet_state() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet("worker-1", "worker", PacketState::InProgress));
        repo.register_packet(make_packet(
            "reviewer-1",
            "reviewer",
            PacketState::AwaitingReview,
        ));
        repo.register_packet(make_packet(
            "gatekeeper-1",
            "gatekeeper",
            PacketState::Reviewed,
        ));
        repo.register_packet(make_packet("recovery-1", "recovery", PacketState::Failed));

        let worker = repo
            .claim_next(PacketClaimRole::Worker)
            .expect("worker claim should exist");
        let reviewer = repo
            .claim_next(PacketClaimRole::Reviewer)
            .expect("reviewer claim should exist");
        let gatekeeper = repo
            .claim_next(PacketClaimRole::Gatekeeper)
            .expect("gatekeeper claim should exist");
        let recovery = repo
            .claim_next(PacketClaimRole::Recovery)
            .expect("recovery claim should exist");

        assert_eq!(worker.packet_id, "worker-1");
        assert_eq!(reviewer.packet_id, "reviewer-1");
        assert_eq!(gatekeeper.packet_id, "gatekeeper-1");
        assert_eq!(recovery.packet_id, "recovery-1");
    }

    #[test]
    fn claim_next_returns_none_when_no_packet_matches_role() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_staged_packet("repo-p3"));

        assert!(repo.claim_next(PacketClaimRole::Worker).is_none());
        assert!(repo.claim_next(PacketClaimRole::Reviewer).is_none());
        assert!(repo.claim_next(PacketClaimRole::Gatekeeper).is_none());
        assert!(repo.claim_next(PacketClaimRole::Recovery).is_none());
    }
}
