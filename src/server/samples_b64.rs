use std::path::Path;

use base64::{engine::general_purpose, Engine as _};
use num_complex::{Complex, Complex32, ComplexFloat};

use super::error::IQEngineError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamplesB64 {
    #[serde(rename = "samples")]
    pub samples: String,
    #[serde(rename = "data_type")]
    pub data_type: crate::server::DataType,
    #[serde(rename = "sample_rate", skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f32>,
    #[serde(rename = "center_freq", skip_serializing_if = "Option::is_none")]
    pub center_freq: Option<f32>,
}

impl SamplesB64 {
    pub fn new(samples: String, data_type: crate::server::DataType) -> SamplesB64 {
        SamplesB64 {
            samples,
            data_type,
            sample_rate: None,
            center_freq: None,
        }
    }

    pub fn raw(self) -> Result<Vec<u8>, IQEngineError> {
        Ok(general_purpose::STANDARD.decode(self.samples)?)
    }

    pub fn samples_f32(self) -> Result<Vec<f32>, IQEngineError> {
        let bytes = general_purpose::STANDARD.decode(self.samples)?;
        let v: Vec<f32> = bytes
            .array_chunks::<4>()
            .map(|a| f32::from_le_bytes(*a))
            .collect();
        Ok(v)
    }

    pub fn samples_cf32(self) -> Result<Vec<Complex32>, IQEngineError> {
        let bytes = general_purpose::STANDARD.decode(self.samples)?;
        let v: Vec<Complex32> = bytes
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
        let bytes = general_purpose::STANDARD.decode(self.samples)?;
        let v: Vec<Complex<i16>> = bytes
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
        let bytes = general_purpose::STANDARD.decode(self.samples)?;
        let v: Vec<Complex<i8>> = bytes
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

pub struct SamplesB64Builder {
    samples: Option<String>,
    data_type: Option<crate::server::DataType>,
    sample_rate: Option<f32>,
    center_freq: Option<f32>,
    error: Option<IQEngineError>,
}

impl Default for SamplesB64Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl SamplesB64Builder {
    pub fn new() -> SamplesB64Builder {
        SamplesB64Builder {
            samples: None,
            data_type: None,
            sample_rate: None,
            center_freq: None,
            error: None,
        }
    }

    pub fn same_as(samples: &SamplesB64) -> SamplesB64Builder {
        SamplesB64Builder {
            samples: None,
            data_type: Some(samples.data_type),
            sample_rate: samples.sample_rate,
            center_freq: samples.center_freq,
            error: None,
        }
    }

    pub fn with_samples_cf32(mut self, samples: Vec<Complex32>) -> Self {
        self.data_type = Some(super::DataType::IqSlashCf32Le);
        let o = samples
            .iter()
            .flat_map(|x| [x.re(), x.im()])
            .flat_map(f32::to_le_bytes);
        let o: Vec<u8> = o.collect();
        let o = general_purpose::STANDARD.encode(o);
        self.samples = Some(o);
        self
    }

    pub fn from_wav_file(mut self, filename: &Path) -> Self {
        let content = std::fs::read(filename);
        if let Ok(content) = content {
            let content = general_purpose::STANDARD.encode(content);
            self.samples = Some(content);
        } else {
            self.error = Some(IQEngineError::IOError(content.unwrap_err()));
        }
        self.data_type = Some(super::DataType::AudioSlashWav);
        self
    }

    pub fn from_wav_data(mut self, samples: impl AsRef<[u8]>) -> Self {
        let content = general_purpose::STANDARD.encode(samples);
        self.samples = Some(content);
        self.data_type = Some(super::DataType::AudioSlashWav);
        self
    }

    pub fn from_u8_data(mut self, samples: impl AsRef<[u8]>) -> Self {
        let content = general_purpose::STANDARD.encode(samples);
        self.samples = Some(content);
        self.data_type = Some(super::DataType::ApplicationSlashOctetStream);
        self
    }

    pub fn build(self) -> Result<SamplesB64, IQEngineError> {
        if self.error.is_some() {
            return Err(self.error.unwrap());
        }
        Ok(SamplesB64 {
            samples: self.samples.unwrap(),
            center_freq: self.center_freq,
            data_type: self.data_type.unwrap(),
            sample_rate: self.sample_rate,
        })
    }
}
