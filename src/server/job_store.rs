use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Context, Result};
use super::{JobStatus, Output};

pub struct JobStore {
    jobs_dir: PathBuf,
    results_dir: PathBuf,
}

impl JobStore {
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self> {
        let jobs_dir = base_path.as_ref().join("jobs");
        let results_dir = base_path.as_ref().join("results");

        fs::create_dir_all(&jobs_dir).context("Failed to create jobs directory")?;
        fs::create_dir_all(&results_dir).context("Failed to create results directory")?;

        Ok(Self { jobs_dir, results_dir })
    }

    pub fn save_job_status(&self, status: &JobStatus) -> Result<()> {
        let path = self.jobs_dir.join(format!("{}.json", status.job_id));
        let content = serde_json::to_string_pretty(status)?;
        fs::write(path, content).context("Failed to write job status")?;
        Ok(())
    }

    pub fn get_job_status(&self, job_id: &str) -> Result<JobStatus> {
        let path = self.jobs_dir.join(format!("{}.json", job_id));
        let content = fs::read_to_string(path).context("Job not found")?;
        let status = serde_json::from_str(&content)?;
        Ok(status)
    }

    pub fn save_result(&self, job_id: &str, result: &Output) -> Result<()> {
        let result_dir = self.results_dir.join(job_id);
        fs::create_dir_all(&result_dir).context("Failed to create result subdirectory")?;
        
        let path = result_dir.join(format!("{}.json", job_id));
        let content = serde_json::to_string_pretty(result)?;
        fs::write(path, content).context("Failed to write result")?;
        Ok(())
    }

    pub fn get_result(&self, job_id: &str) -> Result<Output> {
        let path = self.results_dir.join(job_id).join(format!("{}.json", job_id));
        let content = fs::read_to_string(path).context("Result not found")?;
        let result = serde_json::from_str(&content)?;
        Ok(result)
    }
}
