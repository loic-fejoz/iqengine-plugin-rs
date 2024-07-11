#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStatus<I>
where
    I: ToString
{
    pub job_id: I,
    pub function_name: String,
    pub file_name: String,
    pub progress: f32,
    pub error: Option<String>,
}