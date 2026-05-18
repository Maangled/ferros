use std::fmt;
use std::fs;
use std::io;
use std::ops::{Index, IndexMut};
use std::path::{Path, PathBuf};
use std::slice::{Iter, IterMut};

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

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

fn validate_transition_requirements_with_packet_evidence(
    from: &PacketState,
    to: PacketState,
    review_verdict: Option<&ReviewVerdict>,
    gatekeeper_decision: Option<&GatekeeperDecision>,
    packet_has_evidence: bool,
    evidence_refs: &[String],
) -> Result<(), PacketTransitionError> {
    let has_evidence = packet_has_evidence || has_non_empty_evidence_refs(evidence_refs);

    if to == PacketState::AwaitingReview && !has_evidence {
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
        if !has_evidence {
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

pub fn validate_transition_requirements(
    from: &PacketState,
    to: PacketState,
    review_verdict: Option<&ReviewVerdict>,
    gatekeeper_decision: Option<&GatekeeperDecision>,
    evidence_refs: &[String],
) -> Result<(), PacketTransitionError> {
    validate_transition_requirements_with_packet_evidence(
        from,
        to,
        review_verdict,
        gatekeeper_decision,
        false,
        evidence_refs,
    )
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PacketAuditKind {
    #[default]
    Transition,
    EvidenceAppend,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PacketAuditEntry {
    #[serde(default)]
    pub kind: PacketAuditKind,
    pub seq: usize,
    pub from: PacketState,
    pub to: PacketState,
    pub actor: String,
    pub reason: String,
    pub at: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PacketLifecycleOutcome {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_lifecycle_outcome: Option<PacketLifecycleOutcome>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub last_lifecycle_errors: Vec<String>,
    #[serde(default)]
    pub registration_idempotency_key: Option<String>,
    #[serde(default)]
    pub retry_count: usize,
    #[serde(default)]
    pub retry_budget: usize,
    #[serde(default)]
    pub last_failure_retryable: bool,
    #[serde(default)]
    pub lease_role: Option<PacketClaimRole>,
    #[serde(default)]
    pub lease_expires_at: Option<String>,
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

fn packet_has_non_empty_evidence(packet: &MonitorPacket) -> bool {
    packet
        .audit_trail
        .iter()
        .any(|entry| has_non_empty_evidence_refs(&entry.evidence_refs))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketEnqueueRequest {
    pub packet_id: String,
    pub session_id: String,
    pub origin_message_id: Option<String>,
    pub parent_packet_id: Option<String>,
    pub work_order_id: Option<String>,
    pub manager: String,
    pub state: PacketState,
    pub lifecycle_thread_id: Option<String>,
    pub notification_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub summary: String,
    pub registration_idempotency_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketEnqueueResult {
    pub packet_id: String,
    pub created: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketRepositorySnapshot {
    pub packets: Vec<MonitorPacket>,
}

impl PacketRepositorySnapshot {
    pub fn len(&self) -> usize {
        self.packets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketTransitionRequest {
    pub packet_id: String,
    pub to_state: PacketState,
    pub actor: String,
    pub reason: String,
    pub at: String,
    pub idempotency_key: Option<String>,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketEvidenceAppendRequest {
    pub packet_id: String,
    pub actor: String,
    pub reason: String,
    pub at: String,
    pub idempotency_key: Option<String>,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketEvidenceAppendResult {
    pub packet_id: String,
    pub seq: usize,
    pub appended: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
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

const MANAGER_LEASE_SECONDS: i64 = 10;
const WORKER_LEASE_SECONDS: i64 = 30;
const REVIEWER_LEASE_SECONDS: i64 = 5;
const GATEKEEPER_LEASE_SECONDS: i64 = 5;
const RECOVERY_LEASE_SECONDS: i64 = 60;

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

fn lease_duration_seconds(role: PacketClaimRole) -> i64 {
    match role {
        PacketClaimRole::Manager => MANAGER_LEASE_SECONDS,
        PacketClaimRole::Worker => WORKER_LEASE_SECONDS,
        PacketClaimRole::Reviewer => REVIEWER_LEASE_SECONDS,
        PacketClaimRole::Gatekeeper => GATEKEEPER_LEASE_SECONDS,
        PacketClaimRole::Recovery => RECOVERY_LEASE_SECONDS,
    }
}

fn parse_rfc3339(timestamp: &str) -> Result<OffsetDateTime, String> {
    OffsetDateTime::parse(timestamp, &Rfc3339)
        .map_err(|error| format!("invalid RFC3339 timestamp `{timestamp}`: {error}"))
}

fn compute_lease_expiry(at: &str, role: PacketClaimRole) -> Result<String, String> {
    let expires_at = parse_rfc3339(at)? + Duration::seconds(lease_duration_seconds(role));
    expires_at
        .format(&Rfc3339)
        .map_err(|error| format!("failed to format lease expiry: {error}"))
}

fn current_rfc3339_timestamp() -> Result<String, String> {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| format!("failed to format current timestamp: {error}"))
}

fn normalize_idempotency_key(idempotency_key: Option<String>) -> Result<Option<String>, String> {
    let Some(key) = idempotency_key else {
        return Ok(None);
    };
    let trimmed = key.trim();
    if trimmed.is_empty() {
        return Err("idempotency key must not be empty".to_owned());
    }
    Ok(Some(trimmed.to_owned()))
}

fn validate_enqueue_request(request: &PacketEnqueueRequest) -> Result<(), String> {
    if request.packet_id.trim().is_empty() {
        return Err("packet id must not be empty".to_owned());
    }
    if request.session_id.trim().is_empty() {
        return Err("session id must not be empty".to_owned());
    }
    if request.manager.trim().is_empty() {
        return Err("manager must not be empty".to_owned());
    }
    if request.summary.trim().is_empty() {
        return Err("summary must not be empty".to_owned());
    }

    parse_rfc3339(&request.created_at)?;
    parse_rfc3339(&request.updated_at)?;

    Ok(())
}

fn validate_append_evidence_request(request: &PacketEvidenceAppendRequest) -> Result<(), String> {
    if request.packet_id.trim().is_empty() {
        return Err("packet id must not be empty".to_owned());
    }
    if request.actor.trim().is_empty() {
        return Err("actor must not be empty".to_owned());
    }
    if request.reason.trim().is_empty() {
        return Err("reason must not be empty".to_owned());
    }
    if !has_non_empty_evidence_refs(&request.evidence_refs) {
        return Err("evidence append requires at least one non-empty evidence ref".to_owned());
    }

    parse_rfc3339(&request.at)?;

    Ok(())
}

fn packet_from_enqueue_request(
    request: PacketEnqueueRequest,
) -> Result<MonitorPacket, String> {
    validate_enqueue_request(&request)?;

    Ok(MonitorPacket {
        id: request.packet_id,
        session_id: request.session_id,
        origin_message_id: request.origin_message_id,
        parent_packet_id: request.parent_packet_id,
        work_order_id: request.work_order_id,
        manager: request.manager,
        state: request.state,
        review_verdict: None,
        gatekeeper_decision: None,
        lifecycle_thread_id: request.lifecycle_thread_id,
        notification_id: request.notification_id,
        created_at: request.created_at,
        updated_at: request.updated_at,
        summary: request.summary,
        last_error: None,
        last_lifecycle_outcome: None,
        last_lifecycle_errors: vec![],
        registration_idempotency_key: normalize_idempotency_key(
            request.registration_idempotency_key,
        )?,
        retry_count: 0,
        retry_budget: 0,
        last_failure_retryable: false,
        lease_role: None,
        lease_expires_at: None,
        audit_seq: 0,
        audit_trail: vec![],
    })
}

pub trait PacketRepository {
    fn register_packet(&mut self, packet: MonitorPacket) -> Result<(), String>;
    fn enqueue(&mut self, request: PacketEnqueueRequest) -> Result<PacketEnqueueResult, String>;
    fn append_evidence(
        &mut self,
        request: PacketEvidenceAppendRequest,
    ) -> Result<Option<PacketEvidenceAppendResult>, String>;
    fn snapshot(&self) -> PacketRepositorySnapshot;
    fn packet(&self, packet_id: &str) -> Option<&MonitorPacket>;
    fn packet_by_registration_idempotency_key(&self, key: &str) -> Option<&MonitorPacket>;
    fn claim_next(
        &mut self,
        role: PacketClaimRole,
        at: &str,
    ) -> Result<Option<PacketClaim>, String>;
    fn reclaim_expired(&mut self, now: &str) -> Result<Vec<String>, String>;
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

    pub fn register_packet(&mut self, packet: MonitorPacket) -> Result<(), String> {
        self.register_packet_inner(packet)
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
        self.register_packet(packet)
            .expect("in-memory packet registration should succeed");
    }

    fn register_packet_inner(&mut self, mut packet: MonitorPacket) -> Result<(), String> {
        packet.registration_idempotency_key =
            normalize_idempotency_key(packet.registration_idempotency_key)?;
        if let Some(key) = packet.registration_idempotency_key.as_deref() {
            if self.packet_by_registration_idempotency_key(key).is_some() {
                return Ok(());
            }
        }

        self.packets.push(packet);
        Ok(())
    }

    fn enqueue_inner(
        &mut self,
        request: PacketEnqueueRequest,
    ) -> Result<PacketEnqueueResult, String> {
        let packet = packet_from_enqueue_request(request)?;
        let packet_id = packet.id.clone();
        if let Some(key) = packet.registration_idempotency_key.as_deref() {
            if let Some(existing_packet) = self.packet_by_registration_idempotency_key(key) {
                return Ok(PacketEnqueueResult {
                    packet_id: existing_packet.id.clone(),
                    created: false,
                });
            }
        }

        self.packets.push(packet);
        Ok(PacketEnqueueResult {
            packet_id,
            created: true,
        })
    }

    fn packet_by_registration_idempotency_key(&self, key: &str) -> Option<&MonitorPacket> {
        self.packets.iter().find(|packet| {
            packet.registration_idempotency_key.as_deref() == Some(key)
        })
    }

    fn applied_transition_by_idempotency_key(
        packet: &MonitorPacket,
        key: &str,
    ) -> Option<PacketTransitionApplied> {
        packet.audit_trail.iter().find_map(|entry| {
            (entry.kind == PacketAuditKind::Transition
                && entry.idempotency_key.as_deref() == Some(key))
            .then(|| PacketTransitionApplied {
                packet_id: packet.id.clone(),
                from: entry.from.clone(),
                to: entry.to.clone(),
                seq: entry.seq,
            })
        })
    }

    fn appended_evidence_by_idempotency_key(
        packet: &MonitorPacket,
        key: &str,
    ) -> Option<PacketEvidenceAppendResult> {
        packet.audit_trail.iter().find_map(|entry| {
            (entry.kind == PacketAuditKind::EvidenceAppend
                && entry.idempotency_key.as_deref() == Some(key))
            .then(|| PacketEvidenceAppendResult {
                packet_id: packet.id.clone(),
                seq: entry.seq,
                appended: false,
            })
        })
    }

    fn append_evidence_inner(
        &mut self,
        request: PacketEvidenceAppendRequest,
    ) -> Result<Option<PacketEvidenceAppendResult>, String> {
        let Some(packet) = self.packet(&request.packet_id) else {
            return Ok(None);
        };

        validate_append_evidence_request(&request)?;
        let idempotency_key = normalize_idempotency_key(request.idempotency_key.clone())?;
        if let Some(key) = idempotency_key.as_deref() {
            if let Some(existing) = Self::appended_evidence_by_idempotency_key(packet, key) {
                return Ok(Some(existing));
            }
        }

        let seq = packet.audit_seq + 1;
        let state = packet.state.clone();
        let PacketEvidenceAppendRequest {
            packet_id,
            actor,
            reason,
            at,
            idempotency_key: _,
            evidence_refs,
        } = request;
        let audit_entry = PacketAuditEntry {
            kind: PacketAuditKind::EvidenceAppend,
            seq,
            from: state.clone(),
            to: state,
            actor,
            reason,
            at: at.clone(),
            idempotency_key,
            evidence_refs,
        };
        let packet = self
            .packets
            .iter_mut()
            .find(|packet| packet.id == packet_id)
            .expect("packet should still exist during evidence append");
        packet.audit_seq = seq;
        packet.updated_at = at;
        packet.audit_trail.push(audit_entry);

        Ok(Some(PacketEvidenceAppendResult {
            packet_id: packet.id.clone(),
            seq,
            appended: true,
        }))
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
    fn register_packet(&mut self, packet: MonitorPacket) -> Result<(), String> {
        self.register_packet_inner(packet)
    }

    fn enqueue(&mut self, request: PacketEnqueueRequest) -> Result<PacketEnqueueResult, String> {
        self.enqueue_inner(request)
    }

    fn append_evidence(
        &mut self,
        request: PacketEvidenceAppendRequest,
    ) -> Result<Option<PacketEvidenceAppendResult>, String> {
        self.append_evidence_inner(request)
    }

    fn snapshot(&self) -> PacketRepositorySnapshot {
        PacketRepositorySnapshot {
            packets: self.packets.clone(),
        }
    }

    fn packet(&self, packet_id: &str) -> Option<&MonitorPacket> {
        self.packets.iter().find(|packet| packet.id == packet_id)
    }

    fn packet_by_registration_idempotency_key(&self, key: &str) -> Option<&MonitorPacket> {
        self.packet_by_registration_idempotency_key(key)
    }

    fn claim_next(
        &mut self,
        role: PacketClaimRole,
        at: &str,
    ) -> Result<Option<PacketClaim>, String> {
        let lease_expires_at = compute_lease_expiry(at, role)?;
        let Some(packet) = self
            .packets
            .iter_mut()
            .find(|packet| packet_is_claimable_by_role(packet, role) && packet.lease_role.is_none())
        else {
            return Ok(None);
        };

        packet.lease_role = Some(role);
        packet.lease_expires_at = Some(lease_expires_at);
        Ok(Some(PacketClaim {
            packet_id: packet.id.clone(),
            role,
            state: packet.state.clone(),
        }))
    }

    fn reclaim_expired(&mut self, now: &str) -> Result<Vec<String>, String> {
        let now = parse_rfc3339(now)?;
        let mut reclaimed = Vec::new();

        for packet in &mut self.packets {
            let Some(_) = packet.lease_role else {
                continue;
            };
            let expired = match packet.lease_expires_at.as_deref() {
                Some(expires_at) => parse_rfc3339(expires_at)? <= now,
                None => true,
            };
            if !expired {
                continue;
            }

            packet.lease_role = None;
            packet.lease_expires_at = None;
            reclaimed.push(packet.id.clone());
        }

        Ok(reclaimed)
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
            idempotency_key,
            evidence_refs,
        } = transition;

        let (from, seq, review_verdict, gatekeeper_decision, packet_has_evidence) = {
            let Some(packet) = self.packet(&packet_id) else {
                return Ok(None);
            };
            (
                packet.state.clone(),
                packet.audit_seq + 1,
                packet.review_verdict.clone(),
                packet.gatekeeper_decision.clone(),
                packet_has_non_empty_evidence(packet),
            )
        };

        let idempotency_key = normalize_idempotency_key(idempotency_key).map_err(|message| {
            PacketTransitionError {
                from: from.clone(),
                to: to_state.clone(),
                message,
            }
        })?;
        if let Some(key) = idempotency_key.as_deref() {
            if let Some(applied) = Self::applied_transition_by_idempotency_key(
                self.packet(&packet_id)
                    .expect("packet should still exist during idempotent lookup"),
                key,
            ) {
                return Ok(Some(applied));
            }
        }

        validate_transition_requirements_with_packet_evidence(
            &from,
            to_state.clone(),
            review_verdict.as_ref(),
            gatekeeper_decision.as_ref(),
            packet_has_evidence,
            &evidence_refs,
        )?;

        let next = try_transition(&from, to_state, &actor, &reason, &at)?;
        let audit_entry = PacketAuditEntry {
            kind: PacketAuditKind::Transition,
            seq,
            from: from.clone(),
            to: next.clone(),
            actor,
            reason,
            at: at.clone(),
            idempotency_key,
            evidence_refs,
        };
        let packet = self
            .packets
            .iter_mut()
            .find(|packet| packet.id == packet_id)
            .unwrap();
        packet.state = next.clone();
        packet.updated_at = at;
        packet.lease_role = None;
        packet.lease_expires_at = None;
        if next == PacketState::Failed {
            packet.last_error = Some(audit_entry.reason.clone());
        }
        if from == PacketState::Failed && next == PacketState::DispatchedToManager {
            packet.retry_count += 1;
            packet.last_error = None;
            packet.last_lifecycle_outcome = None;
            packet.last_lifecycle_errors.clear();
            packet.notification_id = None;
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
        if packet.state != PacketState::Failed {
            return Err("retry policy can only be set while packet is failed".to_owned());
        }

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

const PACKET_REPOSITORY_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedPacketRepository {
    schema_version: u32,
    packets: InMemoryPacketRepository,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePacketRepository {
    path: PathBuf,
    packets: InMemoryPacketRepository,
}

impl FilePacketRepository {
    pub fn load_or_default(path: impl Into<PathBuf>) -> Result<Self, String> {
        let now = current_rfc3339_timestamp()?;
        Self::load_or_default_at(path.into(), &now)
    }

    pub fn into_inner(self) -> InMemoryPacketRepository {
        self.packets
    }

    pub fn persist_snapshot(
        path: impl Into<PathBuf>,
        packets: &InMemoryPacketRepository,
    ) -> Result<(), String> {
        let path = path.into();
        persist_packet_repository_to(&path, packets)
    }

    fn load_or_default_at(path: PathBuf, now: &str) -> Result<Self, String> {
        let packets = load_packet_repository_from(&path)?;
        let mut repository = Self { path, packets };
        let reclaimed = repository.packets.reclaim_expired(now)?;
        if !reclaimed.is_empty() {
            repository.persist()?;
        }
        Ok(repository)
    }

    fn persist(&self) -> Result<(), String> {
        persist_packet_repository_to(&self.path, &self.packets)
    }
}

fn load_packet_repository_from(path: &Path) -> Result<InMemoryPacketRepository, String> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(InMemoryPacketRepository::default())
        }
        Err(error) => {
            return Err(format!(
                "failed to read packet repository {}: {error}",
                path.display()
            ))
        }
    };

    let persisted: PersistedPacketRepository = serde_json::from_slice(&bytes)
        .map_err(|error| format!("failed to deserialize packet repository {}: {error}", path.display()))?;
    if persisted.schema_version != PACKET_REPOSITORY_SCHEMA_VERSION {
        return Err(format!(
            "unsupported packet repository schema version {}",
            persisted.schema_version
        ));
    }

    Ok(persisted.packets)
}

fn persist_packet_repository_to(
    path: &Path,
    packets: &InMemoryPacketRepository,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create packet repository directory {}: {error}", parent.display()))?;
    }

    let payload = PersistedPacketRepository {
        schema_version: PACKET_REPOSITORY_SCHEMA_VERSION,
        packets: packets.clone(),
    };
    let bytes = serde_json::to_vec_pretty(&payload)
        .map_err(|error| format!("failed to serialize packet repository {}: {error}", path.display()))?;
    let tmp_path = path.with_file_name(format!(
        "{}.tmp",
        path.file_name().unwrap_or_default().to_string_lossy()
    ));
    fs::write(&tmp_path, bytes)
        .map_err(|error| format!("failed to write packet repository temp file {}: {error}", tmp_path.display()))?;
    fs::rename(&tmp_path, path).map_err(|error| {
        format!(
            "failed to replace packet repository {} from temp file {}: {error}",
            path.display(),
            tmp_path.display()
        )
    })
}

impl PacketRepository for FilePacketRepository {
    fn register_packet(&mut self, packet: MonitorPacket) -> Result<(), String> {
        self.packets.register_packet(packet)?;
        self.persist()
    }

    fn enqueue(&mut self, request: PacketEnqueueRequest) -> Result<PacketEnqueueResult, String> {
        let result = self.packets.enqueue(request)?;
        if result.created {
            self.persist()?;
        }
        Ok(result)
    }

    fn append_evidence(
        &mut self,
        request: PacketEvidenceAppendRequest,
    ) -> Result<Option<PacketEvidenceAppendResult>, String> {
        let result = self.packets.append_evidence(request)?;
        if result.as_ref().is_some_and(|result| result.appended) {
            self.persist()?;
        }
        Ok(result)
    }

    fn snapshot(&self) -> PacketRepositorySnapshot {
        self.packets.snapshot()
    }

    fn packet(&self, packet_id: &str) -> Option<&MonitorPacket> {
        self.packets.packet(packet_id)
    }

    fn packet_by_registration_idempotency_key(&self, key: &str) -> Option<&MonitorPacket> {
        self.packets.packet_by_registration_idempotency_key(key)
    }

    fn claim_next(
        &mut self,
        role: PacketClaimRole,
        at: &str,
    ) -> Result<Option<PacketClaim>, String> {
        let claim = self.packets.claim_next(role, at)?;
        if claim.is_some() {
            self.persist()?;
        }
        Ok(claim)
    }

    fn reclaim_expired(&mut self, now: &str) -> Result<Vec<String>, String> {
        let reclaimed = self.packets.reclaim_expired(now)?;
        if !reclaimed.is_empty() {
            self.persist()?;
        }
        Ok(reclaimed)
    }

    fn has_child_packets(&self, packet_id: &str) -> bool {
        self.packets.has_child_packets(packet_id)
    }

    fn apply_transition(
        &mut self,
        transition: PacketTransitionRequest,
    ) -> Result<Option<PacketTransitionApplied>, PacketTransitionError> {
        let applied = self.packets.apply_transition(transition)?;
        if let Some(ref applied_transition) = applied {
            self.persist().map_err(|message| PacketTransitionError {
                from: applied_transition.from.clone(),
                to: applied_transition.to.clone(),
                message,
            })?;
        }
        Ok(applied)
    }

    fn set_review_verdict(
        &mut self,
        packet_id: &str,
        verdict: ReviewVerdict,
        at: String,
    ) -> Result<bool, String> {
        let updated = self.packets.set_review_verdict(packet_id, verdict, at)?;
        if updated {
            self.persist()?;
        }
        Ok(updated)
    }

    fn set_gatekeeper_decision(
        &mut self,
        packet_id: &str,
        decision: GatekeeperDecision,
        at: String,
    ) -> Result<bool, String> {
        let updated = self.packets.set_gatekeeper_decision(packet_id, decision, at)?;
        if updated {
            self.persist()?;
        }
        Ok(updated)
    }

    fn set_retry_policy(
        &mut self,
        packet_id: &str,
        retryable: Option<bool>,
        retry_budget: Option<usize>,
        at: String,
    ) -> Result<bool, String> {
        let updated = self
            .packets
            .set_retry_policy(packet_id, retryable, retry_budget, at)?;
        if updated {
            self.persist()?;
        }
        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{
        try_transition, validate_transition_requirements, FilePacketRepository,
        GatekeeperDecision, InMemoryPacketRepository, MonitorPacket, PacketAuditKind,
        PacketClaimRole, PacketEnqueueRequest, PacketEvidenceAppendRequest,
        PacketLifecycleOutcome, PacketRepository, PacketState, PacketTransitionRequest,
        PersistedPacketRepository, ReviewVerdict, PACKET_REPOSITORY_SCHEMA_VERSION,
    };

    fn unique_temp_repo_path(test_name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("ferros-orchestrator-{test_name}-{nonce}.json"))
    }

    fn cleanup_repo_path(path: &Path) {
        let _ = fs::remove_file(path);
        let tmp_path = path.with_file_name(format!(
            "{}.tmp",
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
        let _ = fs::remove_file(tmp_path);
    }

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
            last_lifecycle_outcome: None,
            last_lifecycle_errors: vec![],
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
        repo.register_packet(make_staged_packet("repo-p1"))
            .expect("packet registration should succeed");

        let result = repo.apply_transition(PacketTransitionRequest {
            packet_id: "repo-p1".to_owned(),
            to_state: PacketState::DispatchedToManager,
            actor: "test-actor".to_owned(),
            reason: "test reason".to_owned(),
            at: "2026-01-01T00:00:01Z".to_owned(),
            idempotency_key: None,
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
        assert_eq!(packet.audit_trail[0].kind, PacketAuditKind::Transition);
    }

    #[test]
    fn in_memory_repository_append_evidence_records_audit_entry_without_state_change() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet(
            "evidence-p1",
            "Software Architect",
            PacketState::InProgress,
        ))
        .expect("packet registration should succeed");

        let result = repo
            .append_evidence(PacketEvidenceAppendRequest {
                packet_id: "evidence-p1".to_owned(),
                actor: "test-actor".to_owned(),
                reason: "attach worker artifact".to_owned(),
                at: "2026-01-01T00:00:01Z".to_owned(),
                idempotency_key: None,
                evidence_refs: vec!["evidence://packet-1".to_owned()],
            })
            .expect("evidence append should succeed")
            .expect("packet should exist");

        assert!(result.appended);
        assert_eq!(result.seq, 1);
        let packet = repo.packet("evidence-p1").unwrap();
        assert_eq!(packet.state, PacketState::InProgress);
        assert_eq!(packet.updated_at, "2026-01-01T00:00:01Z");
        assert_eq!(packet.audit_seq, 1);
        assert_eq!(packet.audit_trail.len(), 1);
        assert_eq!(packet.audit_trail[0].kind, PacketAuditKind::EvidenceAppend);
        assert_eq!(packet.audit_trail[0].from, PacketState::InProgress);
        assert_eq!(packet.audit_trail[0].to, PacketState::InProgress);
        assert_eq!(
            packet.audit_trail[0].evidence_refs,
            vec!["evidence://packet-1".to_owned()]
        );
    }

    #[test]
    fn in_memory_repository_dedupes_evidence_append_by_idempotency_key() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet(
            "evidence-p2",
            "Software Architect",
            PacketState::InProgress,
        ))
        .expect("packet registration should succeed");

        let first = repo
            .append_evidence(PacketEvidenceAppendRequest {
                packet_id: "evidence-p2".to_owned(),
                actor: "test-actor".to_owned(),
                reason: "attach first artifact".to_owned(),
                at: "2026-01-01T00:00:01Z".to_owned(),
                idempotency_key: Some("append-evidence-1".to_owned()),
                evidence_refs: vec!["evidence://packet-2-a".to_owned()],
            })
            .expect("first evidence append should succeed")
            .expect("packet should exist");
        let second = repo
            .append_evidence(PacketEvidenceAppendRequest {
                packet_id: "evidence-p2".to_owned(),
                actor: "test-actor".to_owned(),
                reason: "attach duplicate artifact".to_owned(),
                at: "2026-01-01T00:00:02Z".to_owned(),
                idempotency_key: Some("append-evidence-1".to_owned()),
                evidence_refs: vec!["evidence://packet-2-b".to_owned()],
            })
            .expect("duplicate evidence append should succeed")
            .expect("packet should exist");

        assert!(first.appended);
        assert!(!second.appended);
        assert_eq!(first.seq, 1);
        assert_eq!(second.seq, 1);
        let packet = repo.packet("evidence-p2").unwrap();
        assert_eq!(packet.audit_trail.len(), 1);
        assert_eq!(packet.updated_at, "2026-01-01T00:00:01Z");
        assert_eq!(
            packet.audit_trail[0].idempotency_key.as_deref(),
            Some("append-evidence-1")
        );
    }

    #[test]
    fn in_memory_repository_allows_awaiting_review_with_previously_appended_evidence() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet(
            "evidence-p3",
            "Software Architect",
            PacketState::InProgress,
        ))
        .expect("packet registration should succeed");
        repo.append_evidence(PacketEvidenceAppendRequest {
            packet_id: "evidence-p3".to_owned(),
            actor: "worker-agent".to_owned(),
            reason: "attach worker output".to_owned(),
            at: "2026-01-01T00:00:01Z".to_owned(),
            idempotency_key: None,
            evidence_refs: vec!["evidence://packet-3".to_owned()],
        })
        .expect("evidence append should succeed")
        .expect("packet should exist");

        let applied = repo
            .apply_transition(PacketTransitionRequest {
                packet_id: "evidence-p3".to_owned(),
                to_state: PacketState::AwaitingReview,
                actor: "worker-agent".to_owned(),
                reason: "worker submitted for review".to_owned(),
                at: "2026-01-01T00:00:02Z".to_owned(),
                idempotency_key: None,
                evidence_refs: vec![],
            })
            .expect("transition should succeed")
            .expect("packet should exist");

        assert_eq!(applied.to, PacketState::AwaitingReview);
        let packet = repo.packet("evidence-p3").unwrap();
        assert_eq!(packet.state, PacketState::AwaitingReview);
        assert_eq!(packet.audit_trail.len(), 2);
    }

    #[test]
    fn in_memory_repository_allows_resolved_with_previously_appended_evidence() {
        let mut repo = InMemoryPacketRepository::default();
        let mut packet = make_packet(
            "evidence-p4",
            "Software Architect",
            PacketState::Reviewed,
        );
        packet.review_verdict = Some(ReviewVerdict::Approved);
        packet.gatekeeper_decision = Some(GatekeeperDecision::Close);
        repo.register_packet(packet)
            .expect("packet registration should succeed");
        repo.append_evidence(PacketEvidenceAppendRequest {
            packet_id: "evidence-p4".to_owned(),
            actor: "reviewer-agent".to_owned(),
            reason: "attach final evidence".to_owned(),
            at: "2026-01-01T00:00:01Z".to_owned(),
            idempotency_key: None,
            evidence_refs: vec!["evidence://packet-4".to_owned()],
        })
        .expect("evidence append should succeed")
        .expect("packet should exist");

        let applied = repo
            .apply_transition(PacketTransitionRequest {
                packet_id: "evidence-p4".to_owned(),
                to_state: PacketState::Resolved,
                actor: "gatekeeper-agent".to_owned(),
                reason: "close packet".to_owned(),
                at: "2026-01-01T00:00:02Z".to_owned(),
                idempotency_key: None,
                evidence_refs: vec![],
            })
            .expect("transition should succeed")
            .expect("packet should exist");

        assert_eq!(applied.to, PacketState::Resolved);
        let packet = repo.packet("evidence-p4").unwrap();
        assert_eq!(packet.state, PacketState::Resolved);
        assert_eq!(packet.audit_trail.len(), 2);
    }

    #[test]
    fn in_memory_repository_rejects_review_verdict_outside_awaiting_review() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_staged_packet("repo-p2"))
            .expect("packet registration should succeed");

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
        packet.last_lifecycle_outcome = Some(PacketLifecycleOutcome {
            kind: "handoff_blocked".to_owned(),
            summary: "handoff blocked pending escalation".to_owned(),
            work_order_id: Some("wo-123".to_owned()),
            escalation_id: Some("esc-123".to_owned()),
            target_agent_id: Some("ferros-coding-continuity".to_owned()),
            stop_reason: Some("missing-baton".to_owned()),
        });
        packet.last_lifecycle_errors = vec![
            "missing baton packet".to_owned(),
            "operator approval required".to_owned(),
        ];
        packet.notification_id = Some("ntf-123".to_owned());
        repo.register_packet(packet)
            .expect("packet registration should succeed");

        let result = repo.apply_transition(PacketTransitionRequest {
            packet_id: "failed-p1".to_owned(),
            to_state: PacketState::DispatchedToManager,
            actor: "stub-recovery-agent".to_owned(),
            reason: "recovery requeued retryable packet".to_owned(),
            at: "2026-01-01T00:00:01Z".to_owned(),
            idempotency_key: None,
            evidence_refs: vec![],
        });

        assert!(result.is_ok(), "requeue transition must succeed");
        let packet = repo.packet("failed-p1").unwrap();
        assert_eq!(packet.state, PacketState::DispatchedToManager);
        assert_eq!(packet.retry_count, 1);
        assert_eq!(packet.retry_budget, 2);
        assert!(packet.last_error.is_none());
        assert!(packet.last_lifecycle_outcome.is_none());
        assert!(packet.last_lifecycle_errors.is_empty());
        assert!(packet.notification_id.is_none());
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
        ))
        .expect("packet registration should succeed");

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
    fn in_memory_repository_dedupes_registration_by_idempotency_key() {
        let mut repo = InMemoryPacketRepository::default();
        let mut first = make_staged_packet("idem-p1");
        first.registration_idempotency_key = Some("register-1".to_owned());
        let mut second = make_staged_packet("idem-p2");
        second.registration_idempotency_key = Some("register-1".to_owned());

        repo.register_packet(first)
            .expect("first registration should succeed");
        repo.register_packet(second)
            .expect("duplicate idempotent registration should succeed");

        assert_eq!(repo.len(), 1);
        assert!(repo.packet("idem-p1").is_some());
        assert!(repo.packet("idem-p2").is_none());
    }

    #[test]
    fn in_memory_repository_enqueue_creates_packet_and_dedupes_by_idempotency_key() {
        let mut repo = InMemoryPacketRepository::default();

        let first = repo
            .enqueue(PacketEnqueueRequest {
                packet_id: "enq-p1".to_owned(),
                session_id: "session-1".to_owned(),
                origin_message_id: None,
                parent_packet_id: None,
                work_order_id: Some("wo-1".to_owned()),
                manager: "Software Architect".to_owned(),
                state: PacketState::Staged,
                lifecycle_thread_id: None,
                notification_id: None,
                created_at: "2026-01-01T00:00:00Z".to_owned(),
                updated_at: "2026-01-01T00:00:00Z".to_owned(),
                summary: "enqueue packet".to_owned(),
                registration_idempotency_key: Some("enqueue-key-1".to_owned()),
            })
            .expect("first enqueue should succeed");
        let second = repo
            .enqueue(PacketEnqueueRequest {
                packet_id: "enq-p2".to_owned(),
                session_id: "session-2".to_owned(),
                origin_message_id: None,
                parent_packet_id: None,
                work_order_id: Some("wo-2".to_owned()),
                manager: "Software Architect".to_owned(),
                state: PacketState::Staged,
                lifecycle_thread_id: None,
                notification_id: None,
                created_at: "2026-01-01T00:00:01Z".to_owned(),
                updated_at: "2026-01-01T00:00:01Z".to_owned(),
                summary: "duplicate enqueue packet".to_owned(),
                registration_idempotency_key: Some("enqueue-key-1".to_owned()),
            })
            .expect("second enqueue should succeed");

        assert!(first.created);
        assert!(!second.created);
        assert_eq!(first.packet_id, "enq-p1");
        assert_eq!(second.packet_id, "enq-p1");
        assert_eq!(repo.snapshot().len(), 1);
        assert_eq!(
            repo.packet("enq-p1")
                .expect("packet should exist")
                .registration_idempotency_key
                .as_deref(),
            Some("enqueue-key-1")
        );
    }

    #[test]
    fn repository_snapshot_returns_cloned_packets() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_staged_packet("snapshot-p1"))
            .expect("packet registration should succeed");
        repo.register_packet(make_packet(
            "snapshot-p2",
            "Software Architect",
            PacketState::DispatchedToManager,
        ))
        .expect("packet registration should succeed");

        let snapshot = repo.snapshot();

        assert_eq!(snapshot.len(), 2);
        assert_eq!(snapshot.packets[0].id, "snapshot-p1");
        assert_eq!(snapshot.packets[1].id, "snapshot-p2");
    }

    #[test]
    fn in_memory_repository_dedupes_transition_by_idempotency_key() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_staged_packet("idem-t1"))
            .expect("packet registration should succeed");

        let first = repo
            .apply_transition(PacketTransitionRequest {
                packet_id: "idem-t1".to_owned(),
                to_state: PacketState::DispatchedToManager,
                actor: "test-actor".to_owned(),
                reason: "dispatch packet".to_owned(),
                at: "2026-01-01T00:00:01Z".to_owned(),
                idempotency_key: Some("transition-1".to_owned()),
                evidence_refs: vec![],
            })
            .expect("first transition should succeed")
            .expect("packet should exist");
        let second = repo
            .apply_transition(PacketTransitionRequest {
                packet_id: "idem-t1".to_owned(),
                to_state: PacketState::DispatchedToManager,
                actor: "test-actor".to_owned(),
                reason: "dispatch packet".to_owned(),
                at: "2026-01-01T00:00:02Z".to_owned(),
                idempotency_key: Some("transition-1".to_owned()),
                evidence_refs: vec![],
            })
            .expect("duplicate transition should succeed")
            .expect("packet should still resolve to prior transition");

        assert_eq!(first, second);
        let packet = repo.packet("idem-t1").unwrap();
        assert_eq!(packet.state, PacketState::DispatchedToManager);
        assert_eq!(packet.audit_trail.len(), 1);
        assert_eq!(
            packet.audit_trail[0].idempotency_key.as_deref(),
            Some("transition-1")
        );
    }

    #[test]
    fn in_memory_transition_idempotency_is_scoped_per_packet() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_staged_packet("idem-scope-1"))
            .expect("packet registration should succeed");
        repo.register_packet(make_staged_packet("idem-scope-2"))
            .expect("packet registration should succeed");

        let first = repo
            .apply_transition(PacketTransitionRequest {
                packet_id: "idem-scope-1".to_owned(),
                to_state: PacketState::DispatchedToManager,
                actor: "test-actor".to_owned(),
                reason: "dispatch first packet".to_owned(),
                at: "2026-01-01T00:00:01Z".to_owned(),
                idempotency_key: Some("transition-scope-1".to_owned()),
                evidence_refs: vec![],
            })
            .expect("first transition should succeed")
            .expect("packet should exist");
        let second = repo
            .apply_transition(PacketTransitionRequest {
                packet_id: "idem-scope-2".to_owned(),
                to_state: PacketState::DispatchedToManager,
                actor: "test-actor".to_owned(),
                reason: "dispatch second packet".to_owned(),
                at: "2026-01-01T00:00:02Z".to_owned(),
                idempotency_key: Some("transition-scope-1".to_owned()),
                evidence_refs: vec![],
            })
            .expect("second transition should succeed")
            .expect("packet should exist");

        assert_ne!(first.packet_id, second.packet_id);
        let first_packet = repo.packet("idem-scope-1").unwrap();
        let second_packet = repo.packet("idem-scope-2").unwrap();
        assert_eq!(first_packet.audit_trail.len(), 1);
        assert_eq!(second_packet.audit_trail.len(), 1);
        assert_eq!(first_packet.state, PacketState::DispatchedToManager);
        assert_eq!(second_packet.state, PacketState::DispatchedToManager);
    }

    #[test]
    fn in_memory_repository_rejects_retry_policy_for_non_failed_packet() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_staged_packet("repo-p3"))
            .expect("packet registration should succeed");

        let result = repo.set_retry_policy(
            "repo-p3",
            Some(true),
            Some(1),
            "2026-01-01T00:00:01Z".to_owned(),
        );

        assert!(result.is_err(), "retry policy should require failed state");
        assert_eq!(
            result.unwrap_err(),
            "retry policy can only be set while packet is failed"
        );
        let packet = repo.packet("repo-p3").unwrap();
        assert!(!packet.last_failure_retryable);
        assert_eq!(packet.retry_budget, 0);
    }

    #[test]
    fn claim_next_manager_returns_first_dispatched_manager_packet() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet(
            "not-manager",
            "test-manager",
            PacketState::DispatchedToManager,
        ))
        .expect("packet registration should succeed");
        repo.register_packet(make_packet(
            "manager-1",
            "Software Architect",
            PacketState::DispatchedToManager,
        ))
        .expect("packet registration should succeed");
        repo.register_packet(make_packet(
            "manager-2",
            "Business Agent Architect",
            PacketState::DispatchedToManager,
        ))
        .expect("packet registration should succeed");

        let claim = repo
            .claim_next(PacketClaimRole::Manager, "2026-01-01T00:00:00Z")
            .expect("manager claim lookup should succeed")
            .expect("manager claim should exist");

        assert_eq!(claim.packet_id, "manager-1");
        assert_eq!(claim.role, PacketClaimRole::Manager);
        assert_eq!(claim.state, PacketState::DispatchedToManager);
    }

    #[test]
    fn claim_next_does_not_return_same_packet_twice_without_reclaim() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet(
            "manager-1",
            "Software Architect",
            PacketState::DispatchedToManager,
        ))
        .expect("packet registration should succeed");

        let first = repo
            .claim_next(PacketClaimRole::Manager, "2026-01-01T00:00:00Z")
            .expect("first claim lookup should succeed")
            .expect("first claim should exist");
        let second = repo
            .claim_next(PacketClaimRole::Manager, "2026-01-01T00:00:00Z")
            .expect("second claim lookup should succeed");

        assert_eq!(first.packet_id, "manager-1");
        assert!(
            second.is_none(),
            "claimed packet should be leased and unavailable until reclaimed or advanced"
        );
    }

    #[test]
    fn reclaim_expired_returns_packet_to_claimable_pool() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet(
            "manager-1",
            "Software Architect",
            PacketState::DispatchedToManager,
        ))
        .expect("packet registration should succeed");

        let first = repo
            .claim_next(PacketClaimRole::Manager, "2026-01-01T00:00:00Z")
            .expect("first claim lookup should succeed")
            .expect("first claim should exist");
        let reclaimed = repo
            .reclaim_expired("2026-01-01T00:00:10Z")
            .expect("lease reclaim should succeed");
        let second = repo
            .claim_next(PacketClaimRole::Manager, "2026-01-01T00:00:10Z")
            .expect("second claim lookup should succeed")
            .expect("second claim should exist after expiry");

        assert_eq!(first.packet_id, "manager-1");
        assert_eq!(reclaimed, vec!["manager-1".to_owned()]);
        assert_eq!(second.packet_id, "manager-1");
    }

    #[test]
    fn claim_next_routes_other_roles_by_packet_state() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_packet("worker-1", "worker", PacketState::InProgress))
            .expect("packet registration should succeed");
        repo.register_packet(make_packet(
            "reviewer-1",
            "reviewer",
            PacketState::AwaitingReview,
        ))
        .expect("packet registration should succeed");
        repo.register_packet(make_packet(
            "gatekeeper-1",
            "gatekeeper",
            PacketState::Reviewed,
        ))
        .expect("packet registration should succeed");
        repo.register_packet(make_packet("recovery-1", "recovery", PacketState::Failed))
            .expect("packet registration should succeed");

        let worker = repo
            .claim_next(PacketClaimRole::Worker, "2026-01-01T00:00:00Z")
            .expect("worker claim lookup should succeed")
            .expect("worker claim should exist");
        let reviewer = repo
            .claim_next(PacketClaimRole::Reviewer, "2026-01-01T00:00:00Z")
            .expect("reviewer claim lookup should succeed")
            .expect("reviewer claim should exist");
        let gatekeeper = repo
            .claim_next(PacketClaimRole::Gatekeeper, "2026-01-01T00:00:00Z")
            .expect("gatekeeper claim lookup should succeed")
            .expect("gatekeeper claim should exist");
        let recovery = repo
            .claim_next(PacketClaimRole::Recovery, "2026-01-01T00:00:00Z")
            .expect("recovery claim lookup should succeed")
            .expect("recovery claim should exist");

        assert_eq!(worker.packet_id, "worker-1");
        assert_eq!(reviewer.packet_id, "reviewer-1");
        assert_eq!(gatekeeper.packet_id, "gatekeeper-1");
        assert_eq!(recovery.packet_id, "recovery-1");
    }

    #[test]
    fn claim_next_returns_none_when_no_packet_matches_role() {
        let mut repo = InMemoryPacketRepository::default();
        repo.register_packet(make_staged_packet("repo-p3"))
            .expect("packet registration should succeed");

        assert!(repo
            .claim_next(PacketClaimRole::Worker, "2026-01-01T00:00:00Z")
            .expect("worker claim lookup should succeed")
            .is_none());
        assert!(repo
            .claim_next(PacketClaimRole::Reviewer, "2026-01-01T00:00:00Z")
            .expect("reviewer claim lookup should succeed")
            .is_none());
        assert!(repo
            .claim_next(PacketClaimRole::Gatekeeper, "2026-01-01T00:00:00Z")
            .expect("gatekeeper claim lookup should succeed")
            .is_none());
        assert!(repo
            .claim_next(PacketClaimRole::Recovery, "2026-01-01T00:00:00Z")
            .expect("recovery claim lookup should succeed")
            .is_none());
    }

    #[test]
    fn file_packet_repository_round_trips_persisted_transitions() {
        let path = unique_temp_repo_path("roundtrip");

        let mut repo = FilePacketRepository::load_or_default_at(
            path.clone(),
            "2026-01-01T00:00:00Z",
        )
        .expect("file repository should load");
        PacketRepository::register_packet(
            &mut repo,
            make_packet(
                "file-p1",
                "Software Architect",
                PacketState::DispatchedToManager,
            ),
        )
        .expect("packet registration should persist");
        repo.apply_transition(PacketTransitionRequest {
            packet_id: "file-p1".to_owned(),
            to_state: PacketState::InProgress,
            actor: "test-actor".to_owned(),
            reason: "manager claimed packet".to_owned(),
            at: "2026-01-01T00:00:01Z".to_owned(),
            idempotency_key: None,
            evidence_refs: vec![],
        })
        .expect("transition should succeed");
        drop(repo);

        let repo = FilePacketRepository::load_or_default_at(path.clone(), "2026-01-01T00:00:01Z")
            .expect("file repository should reload");
        let packet = repo.packet("file-p1").expect("packet should persist");
        assert_eq!(packet.state, PacketState::InProgress);
        assert_eq!(packet.audit_trail.len(), 1);
        assert!(packet.lease_role.is_none());
        assert!(packet.lease_expires_at.is_none());

        cleanup_repo_path(&path);
    }

    #[test]
    fn file_packet_repository_reclaims_expired_leases_on_load() {
        let path = unique_temp_repo_path("reclaim-load");

        let mut repo = FilePacketRepository::load_or_default_at(
            path.clone(),
            "2026-01-01T00:00:00Z",
        )
        .expect("file repository should load");
        PacketRepository::register_packet(
            &mut repo,
            make_packet(
                "file-p2",
                "Software Architect",
                PacketState::DispatchedToManager,
            ),
        )
        .expect("packet registration should persist");
        let claim = repo
            .claim_next(PacketClaimRole::Manager, "2026-01-01T00:00:00Z")
            .expect("claim should succeed")
            .expect("claim should exist");
        assert_eq!(claim.packet_id, "file-p2");
        drop(repo);

        let mut repo = FilePacketRepository::load_or_default_at(path.clone(), "2026-01-01T00:00:11Z")
            .expect("file repository should reload and reclaim expired lease");
        let packet = repo.packet("file-p2").expect("packet should persist");
        assert!(packet.lease_role.is_none());
        assert!(packet.lease_expires_at.is_none());
        let reclaimed_claim = repo
            .claim_next(PacketClaimRole::Manager, "2026-01-01T00:00:11Z")
            .expect("claim should succeed after reload")
            .expect("claim should be available after expired lease is rebuilt");
        assert_eq!(reclaimed_claim.packet_id, "file-p2");

        cleanup_repo_path(&path);
    }

    #[test]
    fn file_packet_repository_persists_without_tmp_remains() {
        let path = unique_temp_repo_path("persist-atomic");

        let mut repo = FilePacketRepository::load_or_default_at(
            path.clone(),
            "2026-01-01T00:00:00Z",
        )
        .expect("file repository should load");
        PacketRepository::register_packet(&mut repo, make_staged_packet("file-p3"))
            .expect("packet registration should persist");

        assert!(path.exists(), "persisted repository file should exist");
        let tmp_path = path.with_file_name(format!(
            "{}.tmp",
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
        assert!(
            !tmp_path.exists(),
            ".tmp file must not remain after successful repository persist"
        );

        let bytes = fs::read(&path).expect("persisted repository file should be readable");
        let persisted: PersistedPacketRepository =
            serde_json::from_slice(&bytes).expect("persisted repository should deserialize");
        assert_eq!(persisted.schema_version, PACKET_REPOSITORY_SCHEMA_VERSION);
        assert!(persisted.packets.packet("file-p3").is_some());

        cleanup_repo_path(&path);
    }

    #[test]
    fn file_packet_repository_persists_transition_idempotency_across_reload() {
        let path = unique_temp_repo_path("idempotency-reload");

        let mut repo = FilePacketRepository::load_or_default_at(
            path.clone(),
            "2026-01-01T00:00:00Z",
        )
        .expect("file repository should load");
        PacketRepository::register_packet(&mut repo, make_staged_packet("file-p4"))
            .expect("packet registration should persist");
        let first = repo
            .apply_transition(PacketTransitionRequest {
                packet_id: "file-p4".to_owned(),
                to_state: PacketState::DispatchedToManager,
                actor: "test-actor".to_owned(),
                reason: "dispatch packet".to_owned(),
                at: "2026-01-01T00:00:01Z".to_owned(),
                idempotency_key: Some("transition-file-1".to_owned()),
                evidence_refs: vec![],
            })
            .expect("transition should succeed")
            .expect("packet should exist");
        drop(repo);

        let mut repo = FilePacketRepository::load_or_default_at(path.clone(), "2026-01-01T00:00:02Z")
            .expect("file repository should reload");
        let second = repo
            .apply_transition(PacketTransitionRequest {
                packet_id: "file-p4".to_owned(),
                to_state: PacketState::DispatchedToManager,
                actor: "test-actor".to_owned(),
                reason: "dispatch packet".to_owned(),
                at: "2026-01-01T00:00:02Z".to_owned(),
                idempotency_key: Some("transition-file-1".to_owned()),
                evidence_refs: vec![],
            })
            .expect("duplicate transition should succeed")
            .expect("packet should still resolve to prior transition");

        assert_eq!(first, second);
        let packet = repo.packet("file-p4").unwrap();
        assert_eq!(packet.audit_trail.len(), 1);

        cleanup_repo_path(&path);
    }

    #[test]
    fn file_packet_repository_persists_evidence_append_idempotency_across_reload() {
        let path = unique_temp_repo_path("append-evidence-reload");

        let mut repo = FilePacketRepository::load_or_default_at(
            path.clone(),
            "2026-01-01T00:00:00Z",
        )
        .expect("file repository should load");
        PacketRepository::register_packet(
            &mut repo,
            make_packet(
                "file-evidence-p1",
                "Software Architect",
                PacketState::InProgress,
            ),
        )
        .expect("packet registration should persist");
        let first = repo
            .append_evidence(PacketEvidenceAppendRequest {
                packet_id: "file-evidence-p1".to_owned(),
                actor: "test-actor".to_owned(),
                reason: "persist evidence".to_owned(),
                at: "2026-01-01T00:00:01Z".to_owned(),
                idempotency_key: Some("append-evidence-file-1".to_owned()),
                evidence_refs: vec!["evidence://file-p1".to_owned()],
            })
            .expect("evidence append should succeed")
            .expect("packet should exist");
        drop(repo);

        let mut repo = FilePacketRepository::load_or_default_at(path.clone(), "2026-01-01T00:00:02Z")
            .expect("file repository should reload");
        let second = repo
            .append_evidence(PacketEvidenceAppendRequest {
                packet_id: "file-evidence-p1".to_owned(),
                actor: "test-actor".to_owned(),
                reason: "persist evidence".to_owned(),
                at: "2026-01-01T00:00:02Z".to_owned(),
                idempotency_key: Some("append-evidence-file-1".to_owned()),
                evidence_refs: vec!["evidence://file-p1-duplicate".to_owned()],
            })
            .expect("duplicate evidence append should succeed")
            .expect("packet should exist");

        assert_eq!(first.seq, second.seq);
        assert!(!second.appended);
        let packet = repo.packet("file-evidence-p1").unwrap();
        assert_eq!(packet.audit_trail.len(), 1);
        assert_eq!(packet.audit_trail[0].kind, PacketAuditKind::EvidenceAppend);
        assert_eq!(
            packet.audit_trail[0].evidence_refs,
            vec!["evidence://file-p1".to_owned()]
        );

        cleanup_repo_path(&path);
    }
}
