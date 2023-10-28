pub mod annotation;
use std::collections::HashMap;

pub use self::annotation::Annotation;
pub mod data_type;
pub use self::data_type::DataType;
#[allow(non_snake_case)]
pub mod __function_name__get_200_response;
pub use self::__function_name__get_200_response::FunctionNameGet200Response;
#[allow(non_snake_case)]
pub mod __function_name__get_200_response_custom_params_value;
pub use self::__function_name__get_200_response_custom_params_value::FunctionNameGet200ResponseCustomParamsValue;
#[allow(non_snake_case)]
pub mod __function_name__get_200_response_custom_params_value_type;
pub use self::__function_name__get_200_response_custom_params_value_type::FunctionNameGet200ResponseCustomParamsValueType;
#[allow(non_snake_case)]
pub mod __function_name__post_200_response;
pub use self::__function_name__post_200_response::FunctionNamePost200Response;
#[allow(non_snake_case)]
pub mod __function_name__post_request;
pub use self::__function_name__post_request::FunctionNamePostRequest;
pub mod samples_b64;
pub use self::samples_b64::SamplesB64;
pub mod samples_cloud;
pub use self::samples_cloud::SamplesCloud;

pub type FunctionParams = FunctionNameGet200Response;
pub type CustomParamsValue = FunctionNameGet200ResponseCustomParamsValue;
pub type FunctionResult = FunctionNamePost200Response;
pub type CustomParamsList = std::collections::HashMap<String, crate::models::FunctionNameGet200ResponseCustomParamsValue>;

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionParamsBuilder {
    params: FunctionParams,
}

impl Default for FunctionParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionParamsBuilder {
    pub fn new() -> FunctionParamsBuilder {
        let custom_params = HashMap::<String, CustomParamsValue>::new();
        let mut result = FunctionParamsBuilder {
            params: FunctionParams::new(custom_params),
        };
        result.params.max_inputs = Some(1);
        result.params.max_outputs = Some(1);
        result
    }

    pub fn max_inputs(mut self, max_inputs: i32) -> Self {
        self.params.max_inputs = Some(max_inputs);
        self
    }

    pub fn max_outputs(mut self, max_outputs: i32) -> Self {
        self.params.max_outputs = Some(max_outputs);
        self
    }

    pub fn custom_param<S: Into<String>>(
        mut self,
        name: S,
        title: S,
        r#type: CustomParamType,
        default: Option<S>,
    ) -> Self {
        let param = CustomParamsValue {
            title: Some(title.into()),
            default: default.map(|x| x.into()),
            r#type: Some(r#type.to_string()),
        };
        self.params.custom_params.insert(name.into(), param);
        self
    }

    pub fn build(self) -> FunctionParams {
        self.params
    }
}
