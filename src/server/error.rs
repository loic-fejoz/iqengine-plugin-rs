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
    #[error("File IO error")]
    IOError(std::io::Error),
    #[error("FutureSDR error")]
    FutureSDRError(anyhow::Error),
    #[error("Hound error")]
    HoundError(hound::Error),
    #[error("")]
    IntoInnerError(std::io::IntoInnerError<std::io::BufWriter<std::io::Cursor<Vec<u8>>>>),
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
