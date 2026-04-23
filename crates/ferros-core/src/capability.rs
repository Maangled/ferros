use alloc::string::String;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenValidationError {
    Empty,
    ContainsWhitespace,
}

pub(crate) fn validate_token(value: &str) -> Result<(), TokenValidationError> {
    if value.trim().is_empty() {
        return Err(TokenValidationError::Empty);
    }

    if value.chars().any(char::is_whitespace) {
        return Err(TokenValidationError::ContainsWhitespace);
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityError {
    Empty,
    ContainsWhitespace,
}

impl fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "capability cannot be empty"),
            Self::ContainsWhitespace => write!(f, "capability cannot contain whitespace"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Capability(String);

impl Capability {
    pub fn new(value: impl Into<String>) -> Result<Self, CapabilityError> {
        let value = value.into();
        validate_token(&value).map_err(|error| match error {
            TokenValidationError::Empty => CapabilityError::Empty,
            TokenValidationError::ContainsWhitespace => CapabilityError::ContainsWhitespace,
        })?;

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequesterProfileIdError {
    Empty,
    ContainsWhitespace,
}

impl fmt::Display for RequesterProfileIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "requester profile id cannot be empty"),
            Self::ContainsWhitespace => {
                write!(f, "requester profile id cannot contain whitespace")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityRequest {
    requester_profile_id: String,
    capability: Capability,
}

impl CapabilityRequest {
    pub fn new(
        requester_profile_id: impl Into<String>,
        capability: Capability,
    ) -> Result<Self, RequesterProfileIdError> {
        let requester_profile_id = requester_profile_id.into();
        validate_token(&requester_profile_id).map_err(|error| match error {
            TokenValidationError::Empty => RequesterProfileIdError::Empty,
            TokenValidationError::ContainsWhitespace => RequesterProfileIdError::ContainsWhitespace,
        })?;

        Ok(Self {
            requester_profile_id,
            capability,
        })
    }

    #[must_use]
    pub fn requester_profile_id(&self) -> &str {
        &self.requester_profile_id
    }

    #[must_use]
    pub fn capability(&self) -> &Capability {
        &self.capability
    }
}

pub trait CapabilityGrantView {
    fn profile_id(&self) -> &str;

    fn capability(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDenialReason {
    NoGrantsPresented,
    ProfileNotGranted,
    CapabilityNotGranted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allowed,
    Denied(PolicyDenialReason),
}

impl PolicyDecision {
    #[must_use]
    pub const fn is_allowed(self) -> bool {
        matches!(self, Self::Allowed)
    }

    #[must_use]
    pub const fn denial_reason(self) -> Option<PolicyDenialReason> {
        match self {
            Self::Allowed => None,
            Self::Denied(reason) => Some(reason),
        }
    }
}

pub trait PolicyEngine {
    fn evaluate<G>(&self, request: &CapabilityRequest, grants: &[G]) -> PolicyDecision
    where
        G: CapabilityGrantView;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DenyByDefaultPolicy;

impl PolicyEngine for DenyByDefaultPolicy {
    fn evaluate<G>(&self, request: &CapabilityRequest, grants: &[G]) -> PolicyDecision
    where
        G: CapabilityGrantView,
    {
        if grants.is_empty() {
            return PolicyDecision::Denied(PolicyDenialReason::NoGrantsPresented);
        }

        let mut saw_profile = false;

        for grant in grants {
            if grant.profile_id() != request.requester_profile_id() {
                continue;
            }

            saw_profile = true;

            if grant.capability() == request.capability().as_str() {
                return PolicyDecision::Allowed;
            }
        }

        if saw_profile {
            PolicyDecision::Denied(PolicyDenialReason::CapabilityNotGranted)
        } else {
            PolicyDecision::Denied(PolicyDenialReason::ProfileNotGranted)
        }
    }
}