use std::fmt;

use serde::{Deserialize, Serialize};

/// S3 owns the transport boundary, not the wire format. Payloads stay opaque bytes
/// so S4 hosts can map them onto sockets, named pipes, or other local transports.
pub trait BusTransport {
    type Channel: BusChannel<Error = Self::Error>;
    type Listener: BusListener<Channel = Self::Channel, Error = Self::Error>;
    type Error;

    fn kind(&self) -> BusTransportKind;
    fn bind(&self, endpoint: BusEndpoint) -> Result<Self::Listener, Self::Error>;
    fn connect(&self, endpoint: &BusEndpoint) -> Result<Self::Channel, Self::Error>;
}

pub trait BusListener {
    type Channel: BusChannel<Error = Self::Error>;
    type Error;

    fn endpoint(&self) -> &BusEndpoint;
    fn accept(&mut self) -> Result<Self::Channel, Self::Error>;
}

pub trait BusChannel {
    type Error;

    fn endpoint(&self) -> &BusEndpoint;
    fn send(&mut self, payload: &[u8]) -> Result<(), Self::Error>;
    fn receive(&mut self) -> Result<Vec<u8>, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BusTransportKind {
    InMemory,
    UnixDomainSocket,
    NamedPipe,
}

impl fmt::Display for BusTransportKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InMemory => write!(f, "in-memory"),
            Self::UnixDomainSocket => write!(f, "unix-domain-socket"),
            Self::NamedPipe => write!(f, "named-pipe"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BusEndpointError {
    EmptyLocation,
}

impl fmt::Display for BusEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLocation => write!(f, "bus endpoint location cannot be empty"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BusEndpoint {
    transport: BusTransportKind,
    location: String,
}

impl BusEndpoint {
    pub fn new(
        transport: BusTransportKind,
        location: impl Into<String>,
    ) -> Result<Self, BusEndpointError> {
        let location = location.into();
        if location.trim().is_empty() {
            return Err(BusEndpointError::EmptyLocation);
        }

        Ok(Self {
            transport,
            location,
        })
    }

    #[must_use]
    pub fn transport(&self) -> BusTransportKind {
        self.transport
    }

    #[must_use]
    pub fn location(&self) -> &str {
        &self.location
    }
}

#[cfg(test)]
mod tests {
    use super::{BusEndpoint, BusEndpointError, BusTransportKind};

    #[test]
    fn endpoint_rejects_empty_locations() {
        assert_eq!(
            BusEndpoint::new(BusTransportKind::NamedPipe, "   "),
            Err(BusEndpointError::EmptyLocation)
        );
    }

    #[test]
    fn endpoint_preserves_transport_and_location() {
        let endpoint =
            BusEndpoint::new(BusTransportKind::NamedPipe, r"\\.\pipe\ferros\agents\echo")
                .expect("named pipe endpoint should be valid");

        assert_eq!(endpoint.transport(), BusTransportKind::NamedPipe);
        assert_eq!(endpoint.location(), r"\\.\pipe\ferros\agents\echo");
    }
}
