use super::data_type::DataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataObject {
    pub data_type: Option<DataType>,
    pub file_name: Option<String>,
    pub data: String,
}
