use std::fmt;

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
            | (PacketState::DispatchedToManager, PacketState::HumanInterventionRequired)
            | (PacketState::InProgress, PacketState::AwaitingReview)
            | (PacketState::InProgress, PacketState::Failed)
            | (PacketState::InProgress, PacketState::HumanInterventionRequired)
            | (PacketState::AwaitingReview, PacketState::Reviewed)
            | (PacketState::AwaitingReview, PacketState::HumanInterventionRequired)
            | (PacketState::Reviewed, PacketState::Resolved)
            | (PacketState::Reviewed, PacketState::Failed)
            | (PacketState::Reviewed, PacketState::HumanInterventionRequired)
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
    evidence_refs.iter().any(|reference| !reference.trim().is_empty())
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
                message: "failed transition from reviewed requires gatekeeper decision"
                    .to_owned(),
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
    pub audit_seq: usize,
    #[serde(default)]
    pub audit_trail: Vec<PacketAuditEntry>,
}

#[cfg(test)]
mod tests {
    use super::{
        validate_transition_requirements, try_transition, GatekeeperDecision, PacketState,
        ReviewVerdict,
    };

    #[test]
    fn packet_state_serializes_current_wire_names() {
        fn wire(state: &PacketState) -> String {
            serde_json::to_string(state).expect("PacketState should serialize")
        }

        assert_eq!(wire(&PacketState::DispatchedToManager), "\"dispatched_to_manager\"");
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
            try_transition(from, to, "test-actor", "test reason", "2026-01-01T00:00:00Z")
        };

        assert!(check(&PacketState::Staged, PacketState::DispatchedToManager).is_ok());
        assert!(check(&PacketState::Staged, PacketState::HumanInterventionRequired).is_ok());
        assert!(check(&PacketState::Staged, PacketState::Failed).is_ok());
        assert!(check(&PacketState::DispatchedToManager, PacketState::InProgress).is_ok());
        assert!(
            check(&PacketState::DispatchedToManager, PacketState::HumanInterventionRequired)
                .is_ok()
        );
        assert!(check(&PacketState::InProgress, PacketState::AwaitingReview).is_ok());
        assert!(check(&PacketState::InProgress, PacketState::Failed).is_ok());
        assert!(check(&PacketState::InProgress, PacketState::HumanInterventionRequired).is_ok());
        assert!(check(&PacketState::AwaitingReview, PacketState::Reviewed).is_ok());
        assert!(
            check(&PacketState::AwaitingReview, PacketState::HumanInterventionRequired).is_ok()
        );
        assert!(check(&PacketState::Reviewed, PacketState::Resolved).is_ok());
        assert!(check(&PacketState::Reviewed, PacketState::Failed).is_ok());
        assert!(check(&PacketState::Reviewed, PacketState::HumanInterventionRequired).is_ok());

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
}