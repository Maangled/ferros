use ferros_core::{
    Capability, CapabilityError, CapabilityGrantView, CapabilityRequest, DenyByDefaultPolicy,
    PolicyDecision, PolicyDenialReason, PolicyEngine, RequesterProfileIdError,
};
use proptest::prelude::*;

const TOKEN_BYTES: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789._-";

#[derive(Debug, Clone, Copy)]
struct GrantStub {
    profile_id: &'static str,
    capability: &'static str,
}

impl CapabilityGrantView for GrantStub {
    fn profile_id(&self) -> &str {
        self.profile_id
    }

    fn capability(&self) -> &str {
        self.capability
    }
}

#[derive(Debug, Clone, Copy)]
struct InactiveGrantStub {
    profile_id: &'static str,
    capability: &'static str,
}

impl CapabilityGrantView for InactiveGrantStub {
    fn profile_id(&self) -> &str {
        self.profile_id
    }

    fn capability(&self) -> &str {
        self.capability
    }

    fn is_active(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
struct GeneratedGrant {
    profile_id: String,
    capability: String,
    active: bool,
}

impl CapabilityGrantView for GeneratedGrant {
    fn profile_id(&self) -> &str {
        &self.profile_id
    }

    fn capability(&self) -> &str {
        &self.capability
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

#[derive(Debug, Clone, Copy)]
enum MismatchGrantSpec {
    WrongProfileSameCapability { active: bool },
    WrongProfileOtherCapability { active: bool },
    SameProfileOtherCapability { active: bool },
    InactiveExactMatch,
}

fn request(profile_id: &str, capability: &str) -> CapabilityRequest {
    CapabilityRequest::new(
        profile_id,
        Capability::new(capability).expect("valid capability"),
    )
    .expect("valid requester profile id")
}

fn valid_token_strategy() -> impl Strategy<Value = String> {
    proptest::collection::vec(prop::sample::select(TOKEN_BYTES.to_vec()), 1..=12)
        .prop_map(|bytes| String::from_utf8(bytes).expect("ASCII token"))
}

fn mismatch_grant_strategy() -> impl Strategy<Value = MismatchGrantSpec> {
    proptest::prop_oneof![
        any::<bool>().prop_map(|active| MismatchGrantSpec::WrongProfileSameCapability { active }),
        any::<bool>().prop_map(|active| MismatchGrantSpec::WrongProfileOtherCapability { active }),
        any::<bool>().prop_map(|active| MismatchGrantSpec::SameProfileOtherCapability { active }),
        Just(MismatchGrantSpec::InactiveExactMatch),
    ]
}

fn alternate_token(value: &str, suffix: &str) -> String {
    let mut token = String::with_capacity(value.len() + suffix.len() + 1);
    token.push_str(value);
    token.push('.');
    token.push_str(suffix);
    token
}

fn expected_mismatch_decision(profile_id: &str, grants: &[GeneratedGrant]) -> PolicyDecision {
    let saw_active_grant = grants.iter().any(|grant| grant.active);
    let saw_profile = grants
        .iter()
        .any(|grant| grant.active && grant.profile_id == profile_id);

    if !saw_active_grant {
        PolicyDecision::Denied(PolicyDenialReason::NoGrantsPresented)
    } else if saw_profile {
        PolicyDecision::Denied(PolicyDenialReason::CapabilityNotGranted)
    } else {
        PolicyDecision::Denied(PolicyDenialReason::ProfileNotGranted)
    }
}

#[test]
fn capability_rejects_empty_names() {
    assert_eq!(Capability::new(""), Err(CapabilityError::Empty));
    assert_eq!(Capability::new("   \n"), Err(CapabilityError::Empty));
}

#[test]
fn capability_rejects_whitespace_names() {
    assert_eq!(
        Capability::new("consent read"),
        Err(CapabilityError::ContainsWhitespace)
    );
    assert_eq!(
        Capability::new("consent\tread"),
        Err(CapabilityError::ContainsWhitespace)
    );
}

#[test]
fn capability_accepts_dot_scoped_name() {
    let capability = Capability::new("consent.read").expect("capability should parse");

    assert_eq!(capability.as_str(), "consent.read");
}

#[test]
fn request_rejects_empty_profile_ids() {
    let capability = Capability::new("consent.read").expect("capability should parse");

    assert_eq!(
        CapabilityRequest::new("", capability.clone()),
        Err(RequesterProfileIdError::Empty)
    );
    assert_eq!(
        CapabilityRequest::new("   ", capability),
        Err(RequesterProfileIdError::Empty)
    );
}

#[test]
fn request_rejects_profile_ids_with_whitespace() {
    let capability = Capability::new("consent.read").expect("capability should parse");

    assert_eq!(
        CapabilityRequest::new("profile alpha", capability),
        Err(RequesterProfileIdError::ContainsWhitespace)
    );
}

#[test]
fn deny_when_no_grants_are_present() {
    let policy = DenyByDefaultPolicy;
    let request = request("profile-alpha", "consent.read");

    assert_eq!(
        policy.evaluate::<GrantStub>(&request, &[]),
        PolicyDecision::Denied(PolicyDenialReason::NoGrantsPresented)
    );
}

#[test]
fn allow_when_profile_and_capability_match_exactly() {
    let policy = DenyByDefaultPolicy;
    let request = request("profile-alpha", "consent.read");
    let grants = [GrantStub {
        profile_id: "profile-alpha",
        capability: "consent.read",
    }];

    let decision = policy.evaluate(&request, &grants);

    assert!(decision.is_allowed());
    assert_eq!(decision, PolicyDecision::Allowed);
}

#[test]
fn allow_when_matching_grant_is_not_first() {
    let policy = DenyByDefaultPolicy;
    let request = request("profile-alpha", "consent.read");
    let grants = [
        GrantStub {
            profile_id: "profile-bravo",
            capability: "consent.read",
        },
        GrantStub {
            profile_id: "profile-alpha",
            capability: "consent.read",
        },
    ];

    assert_eq!(policy.evaluate(&request, &grants), PolicyDecision::Allowed);
}

#[test]
fn inactive_grant_does_not_authorize_matching_capability() {
    let policy = DenyByDefaultPolicy;
    let request = request("profile-alpha", "consent.read");
    let grants = [InactiveGrantStub {
        profile_id: "profile-alpha",
        capability: "consent.read",
    }];

    assert_eq!(
        policy.evaluate(&request, &grants),
        PolicyDecision::Denied(PolicyDenialReason::NoGrantsPresented)
    );
}

#[test]
fn deny_when_request_profile_has_no_grants() {
    let policy = DenyByDefaultPolicy;
    let request = request("profile-alpha", "consent.read");
    let grants = [GrantStub {
        profile_id: "profile-bravo",
        capability: "consent.read",
    }];

    assert_eq!(
        policy.evaluate(&request, &grants),
        PolicyDecision::Denied(PolicyDenialReason::ProfileNotGranted)
    );
}

#[test]
fn deny_when_profile_exists_but_capability_is_missing() {
    let policy = DenyByDefaultPolicy;
    let request = request("profile-alpha", "consent.read");
    let grants = [GrantStub {
        profile_id: "profile-alpha",
        capability: "consent.write",
    }];

    let decision = policy.evaluate(&request, &grants);

    assert_eq!(
        decision.denial_reason(),
        Some(PolicyDenialReason::CapabilityNotGranted)
    );
    assert_eq!(
        decision,
        PolicyDecision::Denied(PolicyDenialReason::CapabilityNotGranted)
    );
}

#[test]
fn deny_when_target_capability_only_exists_for_other_profiles() {
    let policy = DenyByDefaultPolicy;
    let request = request("profile-alpha", "consent.read");
    let grants = [
        GrantStub {
            profile_id: "profile-alpha",
            capability: "consent.write",
        },
        GrantStub {
            profile_id: "profile-bravo",
            capability: "consent.read",
        },
    ];

    assert_eq!(
        policy.evaluate(&request, &grants),
        PolicyDecision::Denied(PolicyDenialReason::CapabilityNotGranted)
    );
}

#[test]
fn deny_when_profile_match_does_not_override_invalid_capability_name() {
    assert_eq!(
        Capability::new("consent read"),
        Err(CapabilityError::ContainsWhitespace)
    );
}

proptest::proptest! {
    #[test]
    fn active_exact_match_allows_regardless_of_grant_order(
        profile_id in valid_token_strategy(),
        capability in valid_token_strategy(),
        neighbors in proptest::collection::vec(
            (valid_token_strategy(), valid_token_strategy(), any::<bool>()),
            0..=8,
        ),
    ) {
        let policy = DenyByDefaultPolicy;
        let request = request(&profile_id, &capability);
        let mut grants: Vec<GeneratedGrant> = neighbors
            .into_iter()
            .map(|(profile_id, capability, active)| GeneratedGrant {
                profile_id,
                capability,
                active,
            })
            .collect();

        grants.push(GeneratedGrant {
            profile_id: profile_id.clone(),
            capability: capability.clone(),
            active: true,
        });

        let mut reversed = grants.clone();
        reversed.reverse();

        prop_assert_eq!(policy.evaluate(&request, &grants), PolicyDecision::Allowed);
        prop_assert_eq!(policy.evaluate(&request, &reversed), PolicyDecision::Allowed);
    }

    #[test]
    fn mismatched_grants_deny_regardless_of_order(
        profile_id in valid_token_strategy(),
        capability in valid_token_strategy(),
        mismatch_specs in proptest::collection::vec(mismatch_grant_strategy(), 0..=8),
    ) {
        let policy = DenyByDefaultPolicy;
        let request = request(&profile_id, &capability);
        let other_profile = alternate_token(&profile_id, "other-profile");
        let other_capability = alternate_token(&capability, "other-capability");
        let grants: Vec<GeneratedGrant> = mismatch_specs
            .into_iter()
            .map(|spec| match spec {
                MismatchGrantSpec::WrongProfileSameCapability { active } => GeneratedGrant {
                    profile_id: other_profile.clone(),
                    capability: capability.clone(),
                    active,
                },
                MismatchGrantSpec::WrongProfileOtherCapability { active } => GeneratedGrant {
                    profile_id: other_profile.clone(),
                    capability: other_capability.clone(),
                    active,
                },
                MismatchGrantSpec::SameProfileOtherCapability { active } => GeneratedGrant {
                    profile_id: profile_id.clone(),
                    capability: other_capability.clone(),
                    active,
                },
                MismatchGrantSpec::InactiveExactMatch => GeneratedGrant {
                    profile_id: profile_id.clone(),
                    capability: capability.clone(),
                    active: false,
                },
            })
            .collect();

        let expected = expected_mismatch_decision(&profile_id, &grants);
        let forward = policy.evaluate(&request, &grants);
        let mut reversed = grants.clone();
        reversed.reverse();

        prop_assert!(!forward.is_allowed());
        prop_assert_eq!(forward, expected);
        prop_assert_eq!(policy.evaluate(&request, &reversed), expected);
    }
}
