use serde::Serialize;

mod function_parameter;
pub use function_parameter::{FunctionParameters, FunctionParamsBuilder};

mod custom_param_type;
pub use custom_param_type::CustomParamType;

mod function_post_request;
pub use function_post_request::FunctionPostRequest;

mod function_post_response;
pub use function_post_response::FunctionPostResponse;

mod job_status;
pub use job_status::JobStatus;

mod job_store;
pub use job_store::JobStore;

mod orchestrator;
pub use orchestrator::Orchestrator;

mod server;
pub use server::{PluginServer, configure_plugin};

mod metadata_file;
pub use metadata_file::MetadataFile;

mod metadata_cloud;
pub use metadata_cloud::MetadataCloud;

mod data_object;
pub use data_object::DataObject;

mod output;
pub use output::Output;

mod data_type;
pub use data_type::DataType;

mod annotation;
pub use annotation::Annotation;

pub mod error;
use self::error::IQEngineError;

pub trait IQFunction<P>
where
    P: Serialize + Send + Sync + 'static,
{
    fn parameters(&self) -> FunctionParameters;
    fn apply(
        &self,
        request: FunctionPostRequest<P>,
        samples: Vec<num_complex::Complex32>,
        job_id: String,
        job_store: std::sync::Arc<JobStore>,
    ) -> impl std::future::Future<Output = Result<Output, IQEngineError>> + Send;
}
