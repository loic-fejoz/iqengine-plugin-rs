#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CustomParamType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
}

impl ToString for CustomParamType {
    fn to_string(&self) -> String {
        match self {
            Self::String => String::from("string"),
            Self::Number => String::from("number"),
            Self::Integer => String::from("integer"),
            Self::Boolean => String::from("boolean"),
        }
    }
}
