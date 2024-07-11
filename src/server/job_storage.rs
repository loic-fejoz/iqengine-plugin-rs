use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs::File, io::BufReader};

use super::{IQEngineError, JobStatus, FunctionOutput};

pub trait JobStorage<I>
where
    I: ToString,
{
    fn job_status(&self, job_id: I) -> Result<JobStatus<I>, IQEngineError>;
    fn job_result(&self, job_id: I) -> Result<FunctionOutput<I>, IQEngineError>;
    fn set_status(
        &mut self,
        job_id: I,
        progress: f32,
        error: Option<String>,
    ) -> Result<(), IQEngineError>;
    fn store_result(&mut self, job_id: I, result: FunctionOutput<I>) -> Result<(), IQEngineError>;
}

pub struct InMemoryJobStorage<I>
where
    I: ToString
{
    jobs: HashMap<String, JobStatus<I>>,
    results: HashMap<String, FunctionOutput<I>>,
}

impl <I> InMemoryJobStorage<I>
where 
    I: ToString
{
    pub fn new() -> InMemoryJobStorage<I> {
        InMemoryJobStorage {
            jobs: HashMap::new(),
            results: HashMap::new(),
        }
    }
}

impl <T> JobStorage<T> for InMemoryJobStorage<T>
where
    T: ToString + Clone
{
    fn set_status(
        &mut self,
        job_id: T,
        progress: f32,
        error: Option<String>,
    ) -> Result<(), IQEngineError> {
        let ctx = self.jobs.get_mut(&job_id.to_string());
        if ctx.is_none() {
            return Err(IQEngineError::JobNotFound(job_id.to_string()));
        }
        let ctx = ctx.unwrap();
        ctx.progress = progress;
        if error.is_some() {
            ctx.error = error;
        }
        Ok(())
    }

    fn store_result(&mut self, job_id: T, result: FunctionOutput<T>) -> Result<(), IQEngineError> {
        let _v = self.results.insert(job_id.to_string(), result);
        Ok(())
    }

    fn job_status(&self, job_id: T) -> Result<JobStatus<T>, IQEngineError> {
        if let Some(ctx) = self.jobs.get(&job_id.to_string()) {
            Ok(ctx.clone())
        } else {
            Err(IQEngineError::JobNotFound(job_id.to_string()))
        }
    }

    fn job_result(&self, job_id: T) -> Result<FunctionOutput<T>, IQEngineError> {
        if let Some(result) = self.results.get(&job_id.to_string()) {
            Ok(result.clone())
        } else {
            Err(IQEngineError::JobNotFound(job_id.to_string()))
        }
    }
}

pub struct FileJobStorage {
    jobs_basedir: PathBuf,
    results_basedir: PathBuf,
}

impl FileJobStorage {
    pub fn new() -> FileJobStorage {
        FileJobStorage {
            jobs_basedir: std::path::Path::new("./jobs/").to_owned(),
            results_basedir: std::path::Path::new("./results/").to_owned(),
        }
    }

    fn job_filename(basedir: &Path, job_id: &str) -> PathBuf {
        basedir.join(job_id.to_string() + ".json")
    }

    fn from_file<T>(basedir: &Path, job_id: impl ToString) -> Result<T, IQEngineError>
    where
        T: serde::de::DeserializeOwned,
    {
        let job_id = job_id.to_string();
        let filename = Self::job_filename(basedir, &job_id);
        let file = File::open(filename);
        if file.is_err() {
            return Err(IQEngineError::JobNotFound(job_id));
        }
        let file = file.unwrap();
        let reader = BufReader::new(file);
        let ctx: T = serde_json::from_reader(reader)?;
        Ok(ctx)
    }

    fn to_file<T>(value: &T, basedir: &Path, job_id: impl ToString) -> Result<(), IQEngineError>
    where
        T: ?Sized + serde::Serialize,
    {
        let job_id = job_id.to_string();
        let filename = Self::job_filename(basedir, &job_id);
        let file = File::open(filename);
        if file.is_err() {
            return Err(IQEngineError::JobNotFound(job_id.to_string()));
        }
        let file = file.unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, value)?;
        Ok(())
    }
}

impl <T> JobStorage<T> for FileJobStorage
where
    T: ToString + Clone + serde::ser::Serialize + serde::de::DeserializeOwned
{
    fn set_status(
        &mut self,
        job_id: T,
        progress: f32,
        error: Option<String>,
    ) -> Result<(), IQEngineError> {
        let job_id = job_id.to_string();
        let mut ctx: JobStatus<T> = Self::from_file(&self.jobs_basedir, job_id.clone())?;
        ctx.progress = progress;
        if error.is_some() {
            ctx.error = error;
        }
        Self::to_file(&ctx, &self.jobs_basedir, job_id)?;
        Ok(())
    }

    fn store_result(&mut self, job_id: T, result: FunctionOutput<T>) -> Result<(), IQEngineError> {
        Self::to_file(&result, self.results_basedir.as_path(), job_id)?;
        Ok(())
    }

    fn job_status(&self, job_id: T) -> Result<JobStatus<T>, IQEngineError> {
        let ctx: JobStatus<T> = Self::from_file(&self.jobs_basedir, job_id)?;
        Ok(ctx)
    }

    fn job_result(&self, job_id: T) -> Result<FunctionOutput<T>, IQEngineError> {
        let result: FunctionOutput<T> = Self::from_file(&self.results_basedir, job_id)?;
        Ok(result)
    }
}
