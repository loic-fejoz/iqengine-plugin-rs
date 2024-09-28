#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "data_type")]
    pub data_type: crate::server::DataType,
    #[serde(rename = "sample_rate", skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f32>,
    #[serde(rename = "center_freq", skip_serializing_if = "Option::is_none")]
    pub center_freq: Option<f32>,
    #[serde(rename = "file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

// impl Default for Metadata {
//     fn default() -> Self {
//         Self::new()
//     }
// }
