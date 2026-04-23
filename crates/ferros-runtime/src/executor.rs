use std::collections::VecDeque;
use std::convert::Infallible;

/// Schedules runtime work without coupling S4 to today's pre-G3 agent surface.
pub trait Executor {
    type Job;
    type Error;

    fn submit(&mut self, job: Self::Job) -> Result<(), Self::Error>;

    fn pop_next(&mut self) -> Result<Option<Self::Job>, Self::Error>;

    fn pending_jobs(&self) -> usize;
}

#[derive(Debug, Default)]
pub struct InMemoryExecutor<T> {
    pending: VecDeque<T>,
}

impl<T> InMemoryExecutor<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }
}

impl<T> Executor for InMemoryExecutor<T> {
    type Job = T;
    type Error = Infallible;

    fn submit(&mut self, job: Self::Job) -> Result<(), Self::Error> {
        self.pending.push_back(job);
        Ok(())
    }

    fn pop_next(&mut self) -> Result<Option<Self::Job>, Self::Error> {
        Ok(self.pending.pop_front())
    }

    fn pending_jobs(&self) -> usize {
        self.pending.len()
    }
}