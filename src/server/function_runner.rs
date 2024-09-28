use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use super::{
    FunctionParameters, FunctionRequest1, IQEngineError, IQFunction1, JobStatus, JobStorage,
};

pub struct FunctionRunner<F, P, S, I>
where
    P: serde::ser::Serialize + Send,
    F: IQFunction1<P>,
    S: JobStorage<I>,
    I: ToString + Clone + Send,
{
    storage: Arc<Mutex<S>>,
    func: F,
    _p: PhantomData<P>,
    _i: PhantomData<I>,
}

impl<F, P, S, I> FunctionRunner<F, P, S, I>
where
    P: serde::ser::Serialize + Send,
    F: IQFunction1<P> + Copy,
    S: JobStorage<I>,
    I: ToString + Clone + Send,
{
    pub fn definition(&self) -> FunctionParameters {
        self.func.parameters()
    }

    pub async fn run(
        &mut self,
        request: FunctionRequest1<P>,
        ctx: JobStatus<I>,
    ) -> Result<(), IQEngineError> {
        let id = ctx.job_id.clone();
        let result = self.func.apply(request, ctx).await;
        if let Ok(result) = result {
            self.storage
                .lock()
                .unwrap()
                .store_result(id.clone(), result)?;
            self.storage.lock().unwrap().set_status(id, 100.0, None)?;
        } else {
            self.storage.lock().unwrap().set_status(
                id,
                100.0,
                Some(result.err().expect("expecting error").to_string()),
            )?;
        };
        Ok(())
    }
}
