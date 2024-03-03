use serde::Serialize;

mod function_parameter;
pub use function_parameter::{FunctionParameters, FunctionParamsBuilder};

mod custom_param_type;
pub use custom_param_type::CustomParamType;

mod function_post_request;
pub use function_post_request::FunctionPostRequest;

mod function_post_response;
pub use function_post_response::{FunctionPostResponse, FunctionPostResponseBuilder};

mod samples_b64;
pub use samples_b64::{SamplesB64, SamplesB64Builder};

mod samples_cloud;
pub use samples_cloud::SamplesCloud;

mod data_type;
pub use data_type::DataType;

mod annotation;
pub use annotation::Annotation;

pub mod error;
use self::error::IQEngineError;

pub trait IQFunction<P>
where
    P: Serialize,
{
    fn parameters(self) -> FunctionParameters;
    fn apply(
        self,
        request: FunctionPostRequest<P>,
    ) -> impl std::future::Future<Output = Result<FunctionPostResponse, IQEngineError>> + Send;
}
