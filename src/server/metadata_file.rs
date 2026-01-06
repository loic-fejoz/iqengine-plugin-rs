use serde::{Deserialize, Serialize};
use super::DataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataFile {
    pub file_name: String,
    pub data_type: DataType,
    pub sample_rate: f32,
    pub center_freq: f32,
}

impl MetadataFile {
    pub fn new(
        file_name: impl Into<String>,
        data_type: DataType,
        sample_rate: f32,
        center_freq: f32,
    ) -> Self {
        Self {
            file_name: file_name.into(),
            data_type,
            sample_rate,
            center_freq,
        }
    }
}
