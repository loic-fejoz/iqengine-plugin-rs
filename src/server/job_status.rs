use super::IQEngineError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStatus<I>
where
    I: ToString + Send,
{
    pub job_id: I,
    pub function_name: String,
    pub file_name: String,
    pub progress: f32,
    pub error: Option<String>,
}

impl<I> JobStatus<I>
where
    I: ToString + Send,
{
    pub fn new(job_id: I) -> Self {
        Self {
            job_id,
            function_name: Default::default(),
            file_name: Default::default(),
            progress: 0.0,
            error: None,
        }
    }
}

impl<I> JobStatus<I>
where
    I: ToString + Send + Default,
{
    pub fn error(err: IQEngineError) -> Self {
        Self {
            job_id: I::default(),
            function_name: Default::default(),
            file_name: Default::default(),
            progress: 100.0,
            error: Some(err.to_string()),
        }
    }
}
