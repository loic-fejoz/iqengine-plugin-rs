use super::{JobStatus, DataObject};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionOutput<I>
where 
    I: ToString
{
    pub job_status: JobStatus<I>,
    //TODO
    pub non_iq_output_data: Option<DataObject>,
    pub error: Option<String>,
}