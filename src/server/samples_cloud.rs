use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use serde_json::Value;

use super::error::IQEngineError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamplesCloud {
    #[serde(rename = "account_name")]
    pub account_name: String,
    #[serde(rename = "container_name")]
    pub container_name: String,
    #[serde(rename = "file_path")]
    pub file_path: String,
    #[serde(rename = "sas_token", skip_serializing_if = "Option::is_none")]
    pub sas_token: Option<String>,
    #[serde(rename = "byte_offset", skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<i32>,
    #[serde(rename = "byte_length", skip_serializing_if = "Option::is_none")]
    pub byte_length: Option<i32>,
    #[serde(rename = "data_type")]
    pub data_type: crate::server::DataType,
    #[serde(rename = "sample_rate", skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f32>,
    #[serde(rename = "center_freq", skip_serializing_if = "Option::is_none")]
    pub center_freq: Option<f32>,
}

impl SamplesCloud {
    pub fn new(
        account_name: String,
        container_name: String,
        file_path: String,
        data_type: crate::server::DataType,
    ) -> SamplesCloud {
        SamplesCloud {
            account_name,
            container_name,
            file_path,
            sas_token: None,
            byte_offset: None,
            byte_length: None,
            data_type,
            sample_rate: None,
            center_freq: None,
        }
    }

    pub async fn sigmf(&self) -> Result<Value, IQEngineError> {
        let account = self.account_name.clone();
        let container = self.container_name.clone();
        let blob_name = self.file_path.clone() + ".sigmf-data";
        let storage_credentials = if let Some(sas_token) = self.sas_token.clone() {
            StorageCredentials::sas_token(sas_token)?
        } else {
            StorageCredentials::anonymous()
        };
        let blob_client =
            ClientBuilder::new(account, storage_credentials).blob_client(&container, blob_name);
        let mut result: Vec<u8> = vec![];

        println!("Ready to download sigmf-data file...");
        // The stream is composed of individual calls to the get blob endpoint
        let mut stream = blob_client.get().into_stream();
        while let Some(value) = stream.next().await {
            println!("new body...");
            let mut body = value?.data;
            // For each response, we stream the body instead of collecting it all
            // into one large allocation.
            while let Some(value) = body.next().await {
                let value = value?;
                result.extend(&value);
            }
        }
        println!("Ready to parse sigmf-data file...");
        let json: Value = serde_json::from_slice(&result)?;
        println!("{:?}", json);
        Ok(json)
    }
}
