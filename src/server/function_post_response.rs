use super::SamplesB64;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionPostResponse {
    #[serde(rename = "data_output", skip_serializing_if = "Option::is_none")]
    pub data_output: Option<Vec<crate::server::SamplesB64>>,
    // #[serde(rename = "samples_cloud", skip_serializing_if = "Option::is_none")]
    // pub samples_cloud: Option<Vec<crate::server::SamplesCloud>>,
    /// See https://github.com/sigmf/SigMF/blob/sigmf-v1.x/sigmf-spec.md#annotations-array
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<crate::server::Annotation>>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl FunctionPostResponse {
    pub fn new() -> FunctionPostResponse {
        FunctionPostResponse {
            data_output: None,
            annotations: None,
            details: None,
        }
    }
}

pub struct FunctionPostResponseBuilder {
    resp: FunctionPostResponse,
}

impl FunctionPostResponseBuilder {
    pub fn new() -> FunctionPostResponseBuilder {
        FunctionPostResponseBuilder {
            resp: FunctionPostResponse::new(),
        }
    }

    pub fn details(mut self, failure: impl Into<String>) -> FunctionPostResponseBuilder {
        self.resp.details = Some(failure.into());
        self
    }

    pub fn with_samples_b64(mut self, samples: SamplesB64) -> FunctionPostResponseBuilder {
        self.resp.data_output = Some(vec![samples]);
        self
    }

    pub fn build(self) -> FunctionPostResponse {
        return self.resp;
    }
}
