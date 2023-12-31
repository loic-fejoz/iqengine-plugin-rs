#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    /// A human-readable comment
    #[serde(rename = "core:comment", skip_serializing_if = "Option::is_none")]
    pub core_colon_comment: Option<String>,
    /// The frequency (Hz) of the lower edge of the feature described by this annotation.
    #[serde(
        rename = "core:freq_lower_edge",
        skip_serializing_if = "Option::is_none"
    )]
    pub core_colon_freq_lower_edge: Option<f32>,
    /// The frequency (Hz) of the upper edge of the feature described by this annotation.
    #[serde(
        rename = "core:freq_upper_edge",
        skip_serializing_if = "Option::is_none"
    )]
    pub core_colon_freq_upper_edge: Option<f32>,
    /// Human-readable name of the entity that created this annotation.
    #[serde(rename = "core:generator", skip_serializing_if = "Option::is_none")]
    pub core_colon_generator: Option<String>,
    /// A short form human/machine-readable label for the annotation. CAN BE USED TO LABEL CLASSIFIER OUTPUT
    #[serde(rename = "core:label", skip_serializing_if = "Option::is_none")]
    pub core_colon_label: Option<String>,
    /// The number of samples that this Segment applies to.
    #[serde(rename = "core:sample_count")]
    pub core_colon_sample_count: i32,
    /// The sample index at which this Segment takes effect
    #[serde(rename = "core:sample_start")]
    pub core_colon_sample_start: i32,
    /// RFC-4122 unique identifier.
    #[serde(rename = "core:uuid", skip_serializing_if = "Option::is_none")]
    pub core_colon_uuid: Option<uuid::Uuid>,
}

impl Annotation {
    pub fn new(core_colon_sample_count: i32, core_colon_sample_start: i32) -> Annotation {
        Annotation {
            core_colon_comment: None,
            core_colon_freq_lower_edge: None,
            core_colon_freq_upper_edge: None,
            core_colon_generator: None,
            core_colon_label: None,
            core_colon_sample_count,
            core_colon_sample_start,
            core_colon_uuid: None,
        }
    }
}
