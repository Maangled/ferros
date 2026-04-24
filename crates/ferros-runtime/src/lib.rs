#![forbid(unsafe_code)]

pub mod bus;
pub mod executor;

pub use bus::{InMemoryMessageBus, MessageBus};
pub use executor::{Executor, InMemoryExecutor};
