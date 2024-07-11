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
pub trait IQFunction1<P, I>
where
    P: Serialize,
    I: ToString,
{
    fn parameters(self) -> FunctionParameters;
    fn apply(
        self,
        request: FunctionRequest1<P>,
    ) -> impl std::future::Future<Output = Result<FunctionOutput<I>, IQEngineError>> + Send;
}