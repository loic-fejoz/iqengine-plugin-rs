use std::collections::HashMap;

use super::CustomParamType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionParameterDescription {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionParameters {
    // #[serde(rename = "max_inputs", skip_serializing_if = "Option::is_none")]
    // pub max_inputs: Option<i32>,
    // #[serde(rename = "max_outputs", skip_serializing_if = "Option::is_none")]
    // pub max_outputs: Option<i32>,
    #[serde(rename = "custom_params", flatten)]
    pub custom_params: ::std::collections::HashMap<String, FunctionParameterDescription>,
}

impl FunctionParameters {
    pub fn new(
        custom_params: ::std::collections::HashMap<String, FunctionParameterDescription>,
    ) -> FunctionParameters {
        FunctionParameters {
            // max_inputs: None,
            // max_outputs: None,
            custom_params,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionParamsBuilder {
    params: FunctionParameters,
}

impl Default for FunctionParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionParamsBuilder {
    pub fn new() -> FunctionParamsBuilder {
        let custom_params = HashMap::<String, FunctionParameterDescription>::new();
        FunctionParamsBuilder {
            params: FunctionParameters::new(custom_params),
        }
        // result.params.max_inputs = Some(1);
        // result.params.max_outputs = Some(1);
        // result
    }

    pub fn max_inputs(self, _max_inputs: i32) -> Self {
        // self.params.max_inputs = Some(max_inputs);
        self
    }

    pub fn max_outputs(self, _max_outputs: i32) -> Self {
        // self.params.max_outputs = Some(max_outputs);
        self
    }

    pub fn custom_param<S: Into<String>>(
        mut self,
        name: S,
        title: S,
        r#type: CustomParamType,
        default: Option<S>,
    ) -> Self {
        let param = FunctionParameterDescription {
            title: Some(title.into()),
            default: default.map(|x| x.into()),
            r#type: Some(r#type.to_string()),
        };
        self.params.custom_params.insert(name.into(), param);
        self
    }

    pub fn build(self) -> FunctionParameters {
        self.params
    }
}
