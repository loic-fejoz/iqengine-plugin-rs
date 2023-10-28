#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "iq/ci8_le")]
    IqSlashCi8Le,
    #[serde(rename = "iq/ci16_le")]
    IqSlashCi16Le,
    #[serde(rename = "iq/cf32_le")]
    IqSlashCf32Le,
    #[serde(rename = "image/png")]
    ImageSlashPng,
    #[serde(rename = "audio/wav")]
    AudioSlashWav,
    #[serde(rename = "application/octet-stream")]
    ApplicationSlashOctetStream,
    #[serde(rename = "text/plain")]
    TextSlashPlain,
}

impl ToString for DataType {
    fn to_string(&self) -> String {
        match self {
            Self::IqSlashCi8Le => String::from("iq/ci8_le"),
            Self::IqSlashCi16Le => String::from("iq/ci16_le"),
            Self::IqSlashCf32Le => String::from("iq/cf32_le"),
            Self::ImageSlashPng => String::from("image/png"),
            Self::AudioSlashWav => String::from("audio/wav"),
            Self::ApplicationSlashOctetStream => String::from("application/octet-stream"),
            Self::TextSlashPlain => String::from("text/plain"),
        }
    }
}

impl Default for DataType {
    fn default() -> DataType {
        Self::IqSlashCi8Le
    }
}
