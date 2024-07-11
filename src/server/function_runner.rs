use std::marker::PhantomData;

use super::{FunctionParameters, FunctionRequest1, IQEngineError, IQFunction1, JobStatus, JobStorage};

pub struct FunctionRunner<F, P, S, I>
where
    P: serde::ser::Serialize,
    F: IQFunction1<P, I>,
    S: JobStorage<I>,
    I: ToString + Clone,
{
    storage: S,
    func: F,
    _p: PhantomData<P>,
    _i: PhantomData<I>,
}

impl <F, P, S, I> FunctionRunner<F, P, S, I>
where
    P: serde::ser::Serialize,
    F: IQFunction1<P, I> + Copy,
    S: JobStorage<I>,
    I: ToString + Clone,
{
    pub fn definition(&self) -> FunctionParameters {
        self.func.parameters()
    }

    pub async fn run(&mut self, request: FunctionRequest1<P>, ctx: JobStatus<I>) -> Result<(), IQEngineError> {
        let job_id = ctx.job_id;

        let result = self.func.apply(request).await?;
        self.storage.store_result(job_id.clone(), result)?;
        self.storage.set_status(job_id, 100.0, None)?;
        Ok(())
    }
}