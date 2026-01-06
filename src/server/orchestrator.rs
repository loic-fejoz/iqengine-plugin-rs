use std::sync::Arc;
use anyhow::Result;
use super::{JobStatus, JobStore, Output};

pub struct Orchestrator {
    job_store: Arc<JobStore>,
}

impl Orchestrator {
    pub fn new(job_store: Arc<JobStore>) -> Self {
        Self { job_store }
    }

    pub async fn start_job<P, F>(
        &self,
        job_id: String,
        function_name: String,
        file_name: String,
        plugin: Arc<F>,
        request: crate::server::FunctionPostRequest<P>,
        samples: Vec<num_complex::Complex32>,
    ) -> Result<JobStatus>
    where
        P: serde::Serialize + Send + Sync + 'static,
        F: crate::server::IQFunction<P> + Send + Sync + 'static,
    {
        let status = JobStatus::new(job_id.clone(), function_name, file_name);
        self.job_store.save_job_status(&status)?;

        let job_store = self.job_store.clone();
        let job_id_clone = job_id.clone();

        tokio::spawn(async move {
            match plugin.apply(request, samples, job_id_clone.clone(), job_store.clone()).await {
                Ok(result) => {
                    if let Err(e) = job_store.save_result(&job_id_clone, &result) {
                        eprintln!("Failed to save result for job {}: {:?}", job_id_clone, e);
                    }
                    if let Ok(mut status) = job_store.get_job_status(&job_id_clone) {
                        status.progress = 100.0;
                        let _ = job_store.save_job_status(&status);
                    }
                }
                Err(e) => {
                    if let Ok(mut status) = job_store.get_job_status(&job_id_clone) {
                        status.progress = 100.0;
                        status.error = Some(e.to_string());
                        let _ = job_store.save_job_status(&status);
                    }
                }
            }
        });

        Ok(status)
    }

    pub fn get_status(&self, job_id: &str) -> Result<JobStatus> {
        self.job_store.get_job_status(job_id)
    }

    pub fn get_result(&self, job_id: &str) -> Result<Output> {
        self.job_store.get_result(job_id)
    }
}
