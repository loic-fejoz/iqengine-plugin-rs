use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStatus {
    pub job_id: String,
    pub function_name: String,
    pub file_name: String,
    pub progress: f32, // 0 to 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl JobStatus {
    pub fn new(
        job_id: impl Into<String>,
        function_name: impl Into<String>,
        file_name: impl Into<String>,
    ) -> Self {
        Self {
            job_id: job_id.into(),
            function_name: function_name.into(),
            file_name: file_name.into(),
            progress: 0.0,
            error: None,
        }
    }
}
