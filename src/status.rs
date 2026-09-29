use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            Self::Pending => "Pending",
            Self::Running => "Running",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
        };
        write!(f, "{status}")
    }
}