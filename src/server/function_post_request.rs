use crate::server::MetadataFile;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionPostRequest<T>
where
    T: Serialize,
{
    pub metadata_file: MetadataFile,
    pub custom_params: Option<T>,
}

impl<T> FunctionPostRequest<T>
where
    T: Serialize,
{
    pub fn new(metadata_file: MetadataFile, custom_params: Option<T>) -> FunctionPostRequest<T> {
        FunctionPostRequest {
            metadata_file,
            custom_params,
        }
    }
}
