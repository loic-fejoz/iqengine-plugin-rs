use serde::{Deserialize, Serialize};
use super::DataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataObject {
    pub data_type: DataType,
    pub file_name: String,
    pub data: String, // Base64
}

impl DataObject {
    pub fn new(data_type: DataType, file_name: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            data_type,
            file_name: file_name.into(),
            data: data.into(),
        }
    }
}
