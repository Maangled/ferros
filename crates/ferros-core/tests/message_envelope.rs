use ferros_core::{Capability, MessageEnvelope, MessageEnvelopeError};

fn capability() -> Capability {
    Capability::new("runtime.dispatch").expect("capability should parse")
}

#[test]
fn envelope_rejects_empty_sender() {
    let error = MessageEnvelope::new("", "agent.beta", capability(), b"ping".to_vec(), 7)
        .expect_err("empty sender should be rejected");

    assert_eq!(error, MessageEnvelopeError::EmptySender);
}

#[test]
fn envelope_rejects_recipient_with_whitespace() {
    let error = MessageEnvelope::new(
        "agent.alpha",
        "agent beta",
        capability(),
        b"ping".to_vec(),
        7,
    )
    .expect_err("recipient whitespace should be rejected");

    assert_eq!(error, MessageEnvelopeError::RecipientContainsWhitespace);
}

#[test]
fn envelope_accepts_fixed_size_payload_buffer() {
    let envelope = MessageEnvelope::new(
        "agent.alpha",
        "agent.beta",
        capability(),
        [1_u8, 2, 3, 4],
        11,
    )
    .expect("message envelope should accept a fixed-size payload buffer");

    assert_eq!(envelope.payload(), &[1, 2, 3, 4]);
}

#[test]
fn envelope_preserves_message_fields() {
    let envelope = MessageEnvelope::new(
        "agent.alpha",
        "agent.beta",
        capability(),
        b"payload".to_vec(),
        42,
    )
    .expect("message envelope should parse");

    assert_eq!(envelope.sender(), "agent.alpha");
    assert_eq!(envelope.recipient(), "agent.beta");
    assert_eq!(envelope.capability().as_str(), "runtime.dispatch");
    assert_eq!(envelope.payload(), b"payload");
    assert_eq!(envelope.nonce(), 42);
}