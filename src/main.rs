mod status;

use status::JobStatus;

fn main() {
    let s = JobStatus::Pending;
    println!("Status: {s}");
}