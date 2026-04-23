use ferros_core::{
    Capability, CapabilityError, CapabilityGrantView, CapabilityRequest, DenyByDefaultPolicy,
    PolicyDecision, PolicyDenialReason, PolicyEngine, RequesterProfileIdError,
};

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

fn request(profile_id: &str, capability: &str) -> CapabilityRequest {
    CapabilityRequest::new(profile_id, Capability::new(capability).expect("valid capability"))
        .expect("valid requester profile id")
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

    assert_eq!(decision.denial_reason(), Some(PolicyDenialReason::CapabilityNotGranted));
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