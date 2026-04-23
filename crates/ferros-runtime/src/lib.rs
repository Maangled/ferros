#![forbid(unsafe_code)]

pub mod bus;
pub mod executor;

pub use bus::MessageBus;
pub use executor::Executor;