use super::FunctionOutput;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResultResponse<I>
where
    I: ToString + Send,
{
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    value: Option<FunctionOutput<I>>,

    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl<I> JobResultResponse<I>
where
    I: ToString + Send,
{
    pub fn not_found(_job_id: I) -> JobResultResponse<I> {
        JobResultResponse::<I> {
            value: None,
            error: Some("Job not found".to_string()),
        }
    }
}

impl<I> From<FunctionOutput<I>> for JobResultResponse<I>
where
    I: ToString + Send,
{
    fn from(status: FunctionOutput<I>) -> Self {
        if status.job_status.progress < 100.0 {
            JobResultResponse::<I> {
                value: None,
                error: Some("Job not complete".to_string()),
            }
        } else if let Some(err) = status.job_status.error {
            JobResultResponse::<I> {
                value: None,
                error: Some(err),
            }
        } else {
            JobResultResponse::<I> {
                value: Some(status),
                error: None,
            }
        }
    }
}
