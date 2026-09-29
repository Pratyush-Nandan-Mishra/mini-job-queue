use std::fmt;

#[derive(Debug)]
pub enum JobError {
    ExecutionFailed {
        job_id: u32,
        attempts: u32,
        reason: String,
    },
}

impl fmt::Display for JobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutionFailed { job_id, attempts, reason } => {
                write!(f, "Job {job_id} failed after {attempts} attempt(s): {reason}")
            }
        }
    }
}

impl std::error::Error for JobError {}