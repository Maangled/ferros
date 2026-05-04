use alloc::collections::VecDeque;
use core::convert::Infallible;

/// Schedules runtime work without coupling S4 to today's pre-G3 agent surface.
pub trait Executor {
    type Job;
    type Error;

    fn submit(&mut self, job: Self::Job) -> Result<(), Self::Error>;

    fn pop_next(&mut self) -> Result<Option<Self::Job>, Self::Error>;

    fn pending_jobs(&self) -> usize;
}

/// Abstract queue backing for hosted and future non-std executor implementations.
pub trait JobQueue {
    type Job;

    fn push_back(&mut self, job: Self::Job);

    fn pop_front(&mut self) -> Option<Self::Job>;

    fn len(&self) -> usize;
}

#[derive(Debug)]
pub struct DequeJobQueue<T> {
    pending: VecDeque<T>,
}

impl<T> Default for DequeJobQueue<T> {
    fn default() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }
}

impl<T> JobQueue for DequeJobQueue<T> {
    type Job = T;

    fn push_back(&mut self, job: Self::Job) {
        self.pending.push_back(job);
    }

    fn pop_front(&mut self) -> Option<Self::Job> {
        self.pending.pop_front()
    }

    fn len(&self) -> usize {
        self.pending.len()
    }
}

#[derive(Debug)]
pub struct InMemoryExecutor<Q> {
    pending: Q,
}

impl<Q> Default for InMemoryExecutor<Q>
where
    Q: Default,
{
    fn default() -> Self {
        Self {
            pending: Q::default(),
        }
    }
}

impl<T> InMemoryExecutor<DequeJobQueue<T>> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            pending: DequeJobQueue::default(),
        }
    }
}

impl<Q> InMemoryExecutor<Q> {
    #[must_use]
    pub fn from_queue(pending: Q) -> Self {
        Self { pending }
    }
}

impl<Q> Executor for InMemoryExecutor<Q>
where
    Q: JobQueue,
{
    type Job = Q::Job;
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
