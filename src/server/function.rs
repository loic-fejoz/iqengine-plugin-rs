use super::*;

/// The very first synchronous REST API version.
/// It has now been replaced by the asynchronous REST API
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

/// The new version of the API based on Async REST API.
pub trait IQFunction1<P>
where
    P: Serialize + Send,
{
    fn parameters(self) -> FunctionParameters;
    fn apply<I>(
        self,
        request: FunctionRequest1<P>,
        job_status: JobStatus<I>,
    ) -> impl std::future::Future<Output = Result<FunctionOutput<I>, IQEngineError>> + Send
    where
        I: ToString + Send;
}
