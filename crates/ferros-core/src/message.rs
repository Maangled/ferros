use alloc::{boxed::Box, string::String, vec::Vec};
use core::fmt;

use crate::capability::{validate_token, Capability, TokenValidationError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageEnvelopeError {
    EmptySender,
    SenderContainsWhitespace,
    EmptyRecipient,
    RecipientContainsWhitespace,
}

impl fmt::Display for MessageEnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySender => write!(f, "message sender cannot be empty"),
            Self::SenderContainsWhitespace => {
                write!(f, "message sender cannot contain whitespace")
            }
            Self::EmptyRecipient => write!(f, "message recipient cannot be empty"),
            Self::RecipientContainsWhitespace => {
                write!(f, "message recipient cannot contain whitespace")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageEnvelope {
    sender: String,
    recipient: String,
    capability: Capability,
    payload: Box<[u8]>,
    nonce: u64,
}

impl MessageEnvelope {
    pub fn new(
        sender: impl Into<String>,
        recipient: impl Into<String>,
        capability: Capability,
        payload: impl Into<Vec<u8>>,
        nonce: u64,
    ) -> Result<Self, MessageEnvelopeError> {
        let sender = sender.into();
        validate_token(&sender).map_err(|error| match error {
            TokenValidationError::Empty => MessageEnvelopeError::EmptySender,
            TokenValidationError::ContainsWhitespace => {
                MessageEnvelopeError::SenderContainsWhitespace
            }
        })?;

        let recipient = recipient.into();
        validate_token(&recipient).map_err(|error| match error {
            TokenValidationError::Empty => MessageEnvelopeError::EmptyRecipient,
            TokenValidationError::ContainsWhitespace => {
                MessageEnvelopeError::RecipientContainsWhitespace
            }
        })?;

        let payload = payload.into().into_boxed_slice();

        Ok(Self {
            sender,
            recipient,
            capability,
            payload,
            nonce,
        })
    }

    #[must_use]
    pub fn sender(&self) -> &str {
        &self.sender
    }

    #[must_use]
    pub fn recipient(&self) -> &str {
        &self.recipient
    }

    #[must_use]
    pub fn capability(&self) -> &Capability {
        &self.capability
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        self.payload.as_ref()
    }

    #[must_use]
    pub const fn nonce(&self) -> u64 {
        self.nonce
    }
}