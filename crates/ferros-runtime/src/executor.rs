/// Schedules runtime work without coupling S4 to today's pre-G3 agent surface.
pub trait Executor {
    type Job;
    type Error;

    fn submit(&mut self, job: Self::Job) -> Result<(), Self::Error>;

    fn run_next(&mut self) -> Result<bool, Self::Error>;

    fn pending_jobs(&self) -> usize;
}