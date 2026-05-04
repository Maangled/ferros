#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod bus;
pub mod executor;
pub mod local_runway;

pub use bus::{DequeEnvelopeQueue, EnvelopeQueue, InMemoryMessageBus, MessageBus};
pub use executor::{DequeJobQueue, Executor, InMemoryExecutor, JobQueue};
pub use local_runway::{
    LocalRunwayCheckpoint, LocalRunwayIntent, LocalRunwayState, LocalRunwayTransitionError,
};
