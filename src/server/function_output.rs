use super::{DataObject, JobStatus, Metadata};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionOutput<I>
where
    I: ToString + Send,
{
    pub job_status: JobStatus<I>,
    //TODO
    //     metadata_cloud: Optional[MetadataCloud] = None
    //     additionalProperties: Optional[Dict[str, Union[str, float, int, bool]]] = None
    //     annotations: Optional[List[Annotation]] = None
    pub metadata_file: Option<Metadata>,
    pub output_data: Option<String>, // Base64 encoded IQ data
    pub non_iq_output_data: Option<DataObject>, // Base64 encoded data (not IQ) like wav file
    pub error: Option<String>,
}

impl<I> FunctionOutput<I>
where
    I: ToString + Send,
{
    pub fn new(job_status: JobStatus<I>) -> FunctionOutput<I> {
        FunctionOutput::<I> {
            job_status,
            metadata_file: None,
            output_data: None,
            non_iq_output_data: None,
            error: None,
        }
    }
}
