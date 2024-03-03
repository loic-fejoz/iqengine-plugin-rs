use thiserror::Error;

use super::DataType;

#[derive(Debug, Error)]
pub enum IQEngineError {
    #[error("Cannot decode base64 content")]
    Base64Error(#[from] base64::DecodeError),
    #[error("Missing IQ stream")]
    MissingIQError,
    #[error("Not yet implemented")]
    NotYetImplemented(String),
    #[error("DataType is unsupported: {0:?}")]
    UnsupportedDataType(DataType),
    #[error("Mandatory parameter is missing: {0}")]
    MandatoryParameter(String),
    #[error("File IO error: {0}")]
    IOError(std::io::Error),
    #[error("FutureSDR error: {0}")]
    FutureSDRError(anyhow::Error),
    #[error("Azure error: {0}")]
    AzureError(azure_core::error::Error),
    #[error("JSON De/Serialization error: {0}")]
    SerdeJsonError(serde_json::Error),
    #[error("Hound error: {0}")]
    HoundError(hound::Error),
}

impl From<anyhow::Error> for IQEngineError {
    fn from(value: anyhow::Error) -> Self {
        IQEngineError::FutureSDRError(value)
    }
}

impl From<hound::Error> for IQEngineError {
    fn from(value: hound::Error) -> Self {
        IQEngineError::HoundError(value)
    }
}

impl From<azure_core::error::Error> for IQEngineError {
    fn from(value: azure_core::error::Error) -> Self {
        IQEngineError::AzureError(value)
    }
}

impl From<serde_json::Error> for IQEngineError {
    fn from(value: serde_json::Error) -> Self {
        IQEngineError::SerdeJsonError(value)
    }
}
