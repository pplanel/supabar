use std::fmt::Debug;
use thiserror::Error;

pub mod jobs;
pub mod worker;

#[derive(Error, Debug)]
pub enum JobError {
    #[error("Failed to create job (job_id {job_id:?})")]
    CreateFailure { job_id: String },
    #[error("Database error")]
    DatabaseError(),
}
