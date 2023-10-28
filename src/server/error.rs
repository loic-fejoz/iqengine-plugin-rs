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
    #[error("DataType is unsupported")]
    UnsupportedDataType(DataType),
    #[error("Mandatory parameter is missing")]
    MandatoryParameter(String),
}
