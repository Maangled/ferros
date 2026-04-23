#![forbid(unsafe_code)]

extern crate alloc;

pub mod capability;
pub mod message;

pub use capability::{
    Capability, CapabilityError, CapabilityGrantView, CapabilityRequest, DenyByDefaultPolicy,
    PolicyDecision, PolicyDenialReason, PolicyEngine, RequesterProfileIdError,
};
pub use message::{MessageEnvelope, MessageEnvelopeError};

pub const FOUNDATION_MARKER: &str = "foundation-ready";
pub const FOUNDATION_VERSION: &str = "0.1.0-foundation";

#[must_use]
pub const fn foundation_ready() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::{foundation_ready, FOUNDATION_MARKER, FOUNDATION_VERSION};

    #[test]
    fn foundation_marker_is_stable() {
        assert_eq!(FOUNDATION_MARKER, "foundation-ready");
        assert_eq!(FOUNDATION_VERSION, "0.1.0-foundation");
    }

    #[test]
    fn foundation_ready_reports_true() {
        assert!(foundation_ready());
    }
}
