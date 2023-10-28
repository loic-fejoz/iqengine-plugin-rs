use crate::server::SamplesB64;
use crate::server::SamplesCloud;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionPostRequest<T>
where
    T: Serialize,
{
    #[serde(rename = "samples_b64", skip_serializing_if = "Option::is_none")]
    pub samples_b64: Option<Vec<SamplesB64>>,
    #[serde(rename = "samples_cloud", skip_serializing_if = "Option::is_none")]
    pub samples_cloud: Option<Vec<SamplesCloud>>,
    #[serde(
        rename = "custom_params",
        skip_serializing_if = "Option::is_none",
    )]
    pub custom_params: Option<T>,
}

impl<T> FunctionPostRequest<T>
where
    T: Serialize,
{
    pub fn new() -> FunctionPostRequest<T> {
        FunctionPostRequest {
            samples_b64: None,
            samples_cloud: None,
            custom_params: None,
        }
    }
}
