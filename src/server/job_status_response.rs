use super::JobStatus;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStatusResponse<I>
where
    I: ToString,
{
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    value: Option<JobStatus<I>>,

    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl<I> JobStatusResponse<I>
where
    I: ToString,
{
    pub fn not_found(_job_id: I) -> JobStatusResponse<I> {
        JobStatusResponse::<I> {
            value: None,
            error: Some("Job not found".to_string()),
        }
    }
}

impl<I> From<JobStatus<I>> for JobStatusResponse<I>
where
    I: ToString,
{
    fn from(status: JobStatus<I>) -> Self {
        JobStatusResponse::<I> {
            value: Some(status),
            error: None,
        }
    }
}
