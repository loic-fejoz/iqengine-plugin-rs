use super::JobStatus;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionPostResponse {
    #[serde(flatten)]
    pub job_status: JobStatus,
}

impl FunctionPostResponse {
    pub fn new(job_status: JobStatus) -> FunctionPostResponse {
        FunctionPostResponse { job_status }
    }
}

impl From<JobStatus> for FunctionPostResponse {
    fn from(job_status: JobStatus) -> Self {
        Self::new(job_status)
    }
}
