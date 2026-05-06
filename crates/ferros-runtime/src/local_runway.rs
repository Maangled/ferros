use ferros_core::MessageEnvelope;
use crate::{Executor, MessageBus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalRunwayIntent {
    Start,
    Resume,
    Stop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalRunwayCheckpoint {
    Pending,
    ProfileReady,
    ConsentReady,
    RuntimeReady,
    Active,
    Draining,
    Halted,
}

pub type LocalRunwayState = LocalRunwayCheckpoint;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalRunwayTransitionError {
    pub from: LocalRunwayState,
    pub intent: LocalRunwayIntent,
}

#[derive(Debug)]
pub enum LocalRunwayAdapterError<ExecutorError, BusError> {
    Transition(LocalRunwayTransitionError),
    Executor(ExecutorError),
    Bus(BusError),
}

#[derive(Debug)]
pub struct LocalRunwayAdapter<E, B> {
    state: LocalRunwayState,
    executor: E,
    bus: B,
}

impl<E, B> LocalRunwayAdapter<E, B> {
    #[must_use]
    pub const fn with_state(state: LocalRunwayState, executor: E, bus: B) -> Self {
        Self {
            state,
            executor,
            bus,
        }
    }

    #[must_use]
    pub fn new(executor: E, bus: B) -> Self {
        Self::with_state(LocalRunwayState::Pending, executor, bus)
    }

    #[must_use]
    pub const fn state(&self) -> LocalRunwayState {
        self.state
    }

    #[must_use]
    pub fn into_parts(self) -> (LocalRunwayState, E, B) {
        (self.state, self.executor, self.bus)
    }
}

impl<E, B> LocalRunwayAdapter<E, B>
where
    E: Executor,
    B: MessageBus,
{
    pub fn advance(
        &mut self,
        intent: LocalRunwayIntent,
    ) -> Result<LocalRunwayState, LocalRunwayTransitionError> {
        let next = self.state.advance(intent)?;
        self.state = next;
        Ok(next)
    }

    pub fn submit(&mut self, job: E::Job) -> Result<(), E::Error> {
        self.executor.submit(job)
    }

    pub fn route(&mut self, envelope: MessageEnvelope) -> Result<(), B::Error> {
        self.bus.send(envelope)
    }

    pub fn try_recv(&mut self, recipient: &str) -> Result<Option<MessageEnvelope>, B::Error> {
        self.bus.try_recv(recipient)
    }

    pub fn advance_submit_and_route(
        &mut self,
        intent: LocalRunwayIntent,
        job: E::Job,
        envelope: MessageEnvelope,
    ) -> Result<LocalRunwayState, LocalRunwayAdapterError<E::Error, B::Error>> {
        let next = self
            .advance(intent)
            .map_err(LocalRunwayAdapterError::Transition)?;
        self.submit(job).map_err(LocalRunwayAdapterError::Executor)?;
        self.route(envelope).map_err(LocalRunwayAdapterError::Bus)?;
        Ok(next)
    }
}

impl LocalRunwayState {
    pub const ALL: [Self; 7] = [
        Self::Pending,
        Self::ProfileReady,
        Self::ConsentReady,
        Self::RuntimeReady,
        Self::Active,
        Self::Draining,
        Self::Halted,
    ];

    pub const fn advance(
        self,
        intent: LocalRunwayIntent,
    ) -> Result<Self, LocalRunwayTransitionError> {
        match (self, intent) {
            (Self::Pending, LocalRunwayIntent::Start | LocalRunwayIntent::Resume)
            | (Self::Halted, LocalRunwayIntent::Start | LocalRunwayIntent::Resume) => {
                Ok(Self::ProfileReady)
            }
            (Self::ProfileReady, LocalRunwayIntent::Start | LocalRunwayIntent::Resume) => {
                Ok(Self::ConsentReady)
            }
            (Self::ConsentReady, LocalRunwayIntent::Start | LocalRunwayIntent::Resume) => {
                Ok(Self::RuntimeReady)
            }
            (Self::RuntimeReady, LocalRunwayIntent::Start | LocalRunwayIntent::Resume) => {
                Ok(Self::Active)
            }
            (Self::Active, LocalRunwayIntent::Stop) => Ok(Self::Draining),
            (Self::Draining, LocalRunwayIntent::Stop) => Ok(Self::Halted),
            _ => Err(LocalRunwayTransitionError { from: self, intent }),
        }
    }

    #[must_use]
    pub const fn can_observe_local_shell(self) -> bool {
        !matches!(self, Self::Pending)
    }

    #[must_use]
    pub const fn requires_explicit_consent(self) -> bool {
        matches!(
            self,
            Self::ConsentReady | Self::RuntimeReady | Self::Active | Self::Draining
        )
    }

    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Halted)
    }

    #[must_use]
    pub const fn ordinal(self) -> usize {
        match self {
            Self::Pending => 0,
            Self::ProfileReady => 1,
            Self::ConsentReady => 2,
            Self::RuntimeReady => 3,
            Self::Active => 4,
            Self::Draining => 5,
            Self::Halted => 6,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::ProfileReady => "profile-ready",
            Self::ConsentReady => "consent-ready",
            Self::RuntimeReady => "runtime-ready",
            Self::Active => "active",
            Self::Draining => "draining",
            Self::Halted => "halted",
        }
    }

    #[must_use]
    pub const fn shell_detail(self) -> &'static str {
        match self {
            Self::Pending => "Local shell runway not initialized yet.",
            Self::ProfileReady => "Local profile checkpoint observed.",
            Self::ConsentReady => "Profile ready; explicit consent still required.",
            Self::RuntimeReady => "Consent checkpoint observed; runtime activation pending.",
            Self::Active => "Local runtime active on the current host.",
            Self::Draining => "Runtime is draining before halt.",
            Self::Halted => "Local runtime halted cleanly.",
        }
    }
}

#[cfg(test)]
mod tests {
    use ferros_core::{Capability, MessageEnvelope};

    use crate::{Executor, InMemoryExecutor, InMemoryMessageBus, MessageBus};

    use super::{LocalRunwayAdapter, LocalRunwayIntent, LocalRunwayState};

    fn message(recipient: &str, nonce: u64) -> MessageEnvelope {
        MessageEnvelope::new(
            "subcore.boot",
            recipient,
            Capability::new("runtime.dispatch").expect("capability should parse"),
            format!("payload-{nonce}").into_bytes(),
            nonce,
        )
        .expect("message should parse")
    }

    #[test]
    fn start_path_advances_through_the_local_runway_checkpoints() {
        let state = LocalRunwayState::Pending;
        let state = state
            .advance(LocalRunwayIntent::Start)
            .expect("pending -> profile");
        let state = state
            .advance(LocalRunwayIntent::Start)
            .expect("profile -> consent");
        let state = state
            .advance(LocalRunwayIntent::Start)
            .expect("consent -> runtime");
        let state = state
            .advance(LocalRunwayIntent::Start)
            .expect("runtime -> active");

        assert_eq!(state, LocalRunwayState::Active);
        assert!(state.can_observe_local_shell());
        assert!(state.requires_explicit_consent());
    }

    #[test]
    fn stop_path_requires_active_runtime_before_halt() {
        let error = LocalRunwayState::Pending
            .advance(LocalRunwayIntent::Stop)
            .expect_err("pending stop should fail");
        let draining = LocalRunwayState::Active
            .advance(LocalRunwayIntent::Stop)
            .expect("active stop should drain");
        let halted = draining
            .advance(LocalRunwayIntent::Stop)
            .expect("draining stop should halt");

        assert_eq!(error.from, LocalRunwayState::Pending);
        assert_eq!(draining, LocalRunwayState::Draining);
        assert_eq!(halted, LocalRunwayState::Halted);
    }

    #[test]
    fn checkpoint_helpers_expose_stable_order_and_labels() {
        let labels = LocalRunwayState::ALL
            .iter()
            .map(|state| state.as_str())
            .collect::<Vec<_>>();
        let ordinals = LocalRunwayState::ALL
            .iter()
            .map(|state| state.ordinal())
            .collect::<Vec<_>>();

        assert_eq!(
            labels,
            vec![
                "pending",
                "profile-ready",
                "consent-ready",
                "runtime-ready",
                "active",
                "draining",
                "halted",
            ]
        );
        assert_eq!(ordinals, vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn shell_detail_matches_terminal_and_consent_boundaries() {
        assert!(LocalRunwayState::ConsentReady.requires_explicit_consent());
        assert_eq!(
            LocalRunwayState::ConsentReady.shell_detail(),
            "Profile ready; explicit consent still required."
        );
        assert!(!LocalRunwayState::Draining.is_terminal());
        assert!(LocalRunwayState::Halted.is_terminal());
    }

    #[test]
    fn adapter_composes_transition_executor_and_bus_through_runtime_seams() {
        let mut adapter =
            LocalRunwayAdapter::new(InMemoryExecutor::new(), InMemoryMessageBus::new());

        for _ in 0..3 {
            adapter
                .advance(LocalRunwayIntent::Start)
                .expect("start path should advance");
        }

        let next = adapter
            .advance_submit_and_route(
                LocalRunwayIntent::Start,
                "dispatch-message",
                message("subcore.runtime", 19),
            )
            .expect("adapter step should succeed");

        let (_, mut executor, mut bus) = adapter.into_parts();

        assert_eq!(next, LocalRunwayState::Active);
        assert_eq!(executor.pop_next().expect("pop should succeed"), Some("dispatch-message"));
        assert_eq!(executor.pop_next().expect("pop should succeed"), None);

        let delivered = bus
            .try_recv("subcore.runtime")
            .expect("receive should succeed")
            .expect("message should exist");
        assert_eq!(delivered.nonce(), 19);
    }
}
