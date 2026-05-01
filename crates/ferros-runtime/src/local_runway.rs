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
    use super::{LocalRunwayIntent, LocalRunwayState};

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
}
