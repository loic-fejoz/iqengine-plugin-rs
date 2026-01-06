use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

use super::{Annotation, DataObject, JobStatus, MetadataCloud, MetadataFile};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<JobStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_file: Option<MetadataFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_cloud: Option<MetadataCloud>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data: Option<String>, // Base64 for IQ samples
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_output: Option<Vec<DataObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_iq_output_data: Option<DataObject>, // Deprecated in favor of data_output but keeping for now
    
    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}

impl Output {
    pub fn new() -> Self {
        Self {
            job_status: None,
            metadata_file: None,
            metadata_cloud: None,
            annotations: None,
            output_data: None,
            data_output: None,
            non_iq_output_data: None,
            additional_properties: HashMap::new(),
        }
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}
