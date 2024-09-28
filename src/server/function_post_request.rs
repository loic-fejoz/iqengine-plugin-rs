use crate::server::SamplesB64;
use crate::server::SamplesCloud;
use num_complex::Complex;
use num_complex::Complex32;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(feature = "axum")]
use axum::{extract::Multipart, http::StatusCode, Json};

use super::IQEngineError;
#[cfg(feature = "axum")]
use super::JobStatus;
use super::Metadata;

#[deprecated]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionPostRequest<T>
where
    T: Serialize,
{
    #[serde(rename = "samples_b64", skip_serializing_if = "Option::is_none")]
    pub samples_b64: Option<Vec<SamplesB64>>,
    #[serde(rename = "samples_cloud", skip_serializing_if = "Option::is_none")]
    pub samples_cloud: Option<Vec<SamplesCloud>>,
    #[serde(rename = "custom_params", skip_serializing_if = "Option::is_none")]
    pub custom_params: Option<T>,
}

impl<T> FunctionPostRequest<T>
where
    T: Serialize,
{
    pub fn new() -> FunctionPostRequest<T> {
        FunctionPostRequest {
            samples_b64: None,
            samples_cloud: None,
            custom_params: None,
        }
    }
}

impl<T> Default for FunctionPostRequest<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionRequest1Builder<T>
where
    T: Serialize,
{
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "samples", skip_serializing_if = "Option::is_none")]
    pub samples: Option<Vec<u8>>,
    #[serde(rename = "custom_params", skip_serializing_if = "Option::is_none")]
    pub custom_params: Option<T>,
}

impl<'a, T> FunctionRequest1Builder<T>
where
    T: Serialize,
{
    pub fn new() -> FunctionRequest1Builder<T> {
        FunctionRequest1Builder {
            metadata: None,
            samples: None,
            custom_params: None,
        }
    }
}

impl<T> FunctionRequest1Builder<T>
where
    T: Serialize + DeserializeOwned + Clone + Send,
{
    #[cfg(feature = "axum")]
    pub async fn parse<I>(
        mut multipart: Multipart,
    ) -> Result<FunctionRequest1<T>, (StatusCode, Json<JobStatus<I>>)>
    where
        I: ToString + Send + Default,
    {
        let mut req = Self::new();
        while let Some(field) = multipart.next_field().await.unwrap() {
            let name = field.name().unwrap().to_string();
            let data = field.bytes().await.unwrap();

            if name == "iq_file" {
                // IQ File, ie .sigmf-data
                req.samples = Some(data.to_vec());
            } else if name == "metadata_file" {
                // Json like {"file_name":"analog_FM_France.sigmf-data","sample_rate":1920000,"center_freq":96900000,"data_type":"iq/cf32_le"}
                let metadata = serde_json::from_slice::<Metadata>(data.as_ref());
                if let Ok(metadata) = metadata {
                    req.metadata = Some(metadata);
                } else if let Err(err) = metadata {
                    let resp = JobStatus::<I>::error(IQEngineError::SerdeJsonError(err));
                    return Err((StatusCode::BAD_REQUEST, Json(resp)));
                };
            } else if name == "custom_params" {
                // Json like {"target_freq":0}
                let Ok(params) = serde_json::from_slice::<T>(data.as_ref()) else {
                    let resp = JobStatus::<I>::error(IQEngineError::MandatoryParameter(
                        "Cannot get custom_params".to_string(),
                    ));
                    return Err((StatusCode::BAD_REQUEST, Json(resp)));
                };
                req.custom_params = Some(params);
            }
        }
        let Ok(req) = req.build() else {
            let resp = JobStatus::<I>::error(IQEngineError::MandatoryParameter(
                "Cannot build custom_params".to_string(),
            ));
            return Err((StatusCode::BAD_REQUEST, Json(resp)));
        };
        Ok(req)
    }
}

impl<T> Default for FunctionRequest1Builder<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FunctionRequest1Builder<T>
where
    T: Serialize + Send,
{
    pub fn build(self) -> Result<FunctionRequest1<T>, IQEngineError> {
        Ok(FunctionRequest1 {
            metadata: self.metadata.ok_or(IQEngineError::MandatoryParameter(
                "metadata_file is mandatory".to_string(),
            ))?,
            samples: self.samples.ok_or(IQEngineError::MandatoryParameter(
                "iq_file is mandatory".to_string(),
            ))?,
            custom_params: self.custom_params.ok_or(IQEngineError::MandatoryParameter(
                "custom_params are mandatory".to_string(),
            ))?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionRequest1<T>
where
    T: Serialize + Send,
{
    #[serde(rename = "metadata")]
    pub metadata: Metadata,
    #[serde(rename = "samples")]
    pub samples: Vec<u8>,
    #[serde(rename = "custom_params")]
    pub custom_params: T,
}

impl<T> FunctionRequest1<T>
where
    T: Serialize + Send,
{
    pub fn samples_f32(self) -> Result<Vec<f32>, IQEngineError> {
        let v: Vec<f32> = self
            .samples
            .array_chunks::<4>()
            .map(|a| f32::from_le_bytes(*a))
            .collect();
        Ok(v)
    }
    pub fn samples_cf32(self) -> Result<Vec<Complex32>, IQEngineError> {
        let v: Vec<Complex32> = self
            .samples
            .array_chunks::<4>()
            .map(|a| f32::from_le_bytes(*a))
            .array_chunks::<2>()
            .map(|f2| Complex {
                re: f2[0],
                im: f2[1],
            })
            .collect();
        Ok(v)
    }

    pub fn samples_ci16(self) -> Result<Vec<Complex<i16>>, IQEngineError> {
        let v: Vec<Complex<i16>> = self
            .samples
            .array_chunks::<2>()
            .map(|a| i16::from_le_bytes(*a))
            .array_chunks::<2>()
            .map(|f2| Complex {
                re: f2[0],
                im: f2[1],
            })
            .collect();
        Ok(v)
    }

    pub fn samples_ci8(self) -> Result<Vec<Complex<i8>>, IQEngineError> {
        let v: Vec<Complex<i8>> = self
            .samples
            .array_chunks::<1>()
            .map(|a| i8::from_le_bytes(*a))
            .array_chunks::<2>()
            .map(|f2| Complex {
                re: f2[0],
                im: f2[1],
            })
            .collect();
        Ok(v)
    }
}
