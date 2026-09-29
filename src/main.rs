mod error;
mod status;

use error::JobError;
use status::JobStatus;

fn main() {
    let s = JobStatus::Pending;
    println!("Status: {s}");

    let e = JobError::ExecutionFailed {
        job_id: 2,
        attempts: 3,
        reason: "disk full".to_string(),
    };
    println!("Error: {e}");
}