use ferros_core::{
    Capability, CapabilityGrantView, CapabilityRequest, DenyByDefaultPolicy, MessageEnvelope,
    MessageEnvelopeError, PolicyDecision, PolicyDenialReason, PolicyEngine,
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

#[test]
fn foundation_surface_capability_request_preserves_portable_tokens() {
    let capability = Capability::new("boot.observe").expect("capability should parse");
    let request = CapabilityRequest::new("profile-alpha", capability.clone())
        .expect("request should parse");

    assert_eq!(capability.as_str(), "boot.observe");
    assert_eq!(request.requester_profile_id(), "profile-alpha");
    assert_eq!(request.capability().as_str(), "boot.observe");
}

#[test]
fn foundation_surface_message_envelope_round_trip_preserves_payload_and_nonce() {
    let capability = Capability::new("runtime.dispatch").expect("capability should parse");
    let envelope = MessageEnvelope::new(
        "subcore.boot",
        "subcore.runtime",
        capability,
        b"portable-payload".to_vec(),
        42,
    )
    .expect("message should parse");

    assert_eq!(envelope.sender(), "subcore.boot");
    assert_eq!(envelope.recipient(), "subcore.runtime");
    assert_eq!(envelope.capability().as_str(), "runtime.dispatch");
    assert_eq!(envelope.payload(), b"portable-payload");
    assert_eq!(envelope.nonce(), 42);
}

#[test]
fn foundation_surface_rejects_whitespace_sender_and_recipient_tokens() {
    let capability = Capability::new("runtime.dispatch").expect("capability should parse");

    let sender_error = MessageEnvelope::new(
        "subcore boot",
        "subcore.runtime",
        capability.clone(),
        Vec::<u8>::new(),
        1,
    )
    .expect_err("sender with whitespace should fail");
    let recipient_error = MessageEnvelope::new(
        "subcore.boot",
        "subcore runtime",
        capability,
        Vec::<u8>::new(),
        1,
    )
    .expect_err("recipient with whitespace should fail");

    assert_eq!(sender_error, MessageEnvelopeError::SenderContainsWhitespace);
    assert_eq!(recipient_error, MessageEnvelopeError::RecipientContainsWhitespace);
}

#[test]
fn foundation_surface_deny_by_default_rejects_missing_grants() {
    let policy = DenyByDefaultPolicy;
    let request = CapabilityRequest::new(
        "profile-alpha",
        Capability::new("runtime.dispatch").expect("capability should parse"),
    )
    .expect("request should parse");

    let grants: [GrantStub; 0] = [];

    assert_eq!(
        policy.evaluate(&request, &grants),
        PolicyDecision::Denied(PolicyDenialReason::NoGrantsPresented)
    );
}