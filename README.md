# IQEngine Plugin Library (Rust)

A Rust implementation of the IQEngine Plugin API, providing a framework for building asynchronous RF signal processing plugins. This library aligns with the [IQEngine Plugin Specification](https://github.com/IQEngine/IQEngine/blob/main/plugins/src/plugins_api.py).

## Features

- **Asynchronous Execution**: Plugins are executed in background tasks using `tokio`.
- **Job Management**: Built-in `Orchestrator` for handling concurrent jobs, progress reporting, and state persistence.
- **Multipart Support**: Handles `multipart/form-data` for IQ sample uploads, metadata files, and custom parameters.
- **Actix-web Integration**: Pre-built handlers and configuration helpers to quickly expose your plugins via a web server.
- **Signal Metadata**: Comprehensive data models for SIGMF-aligned metadata and annotations.

## API Endpoints

The library provides the following standardized endpoints:

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `GET` | `/plugins` | List available plugin names. |
| `GET` | `/plugins/{name}` | Retrieve plugin parameters and definition. |
| `POST` | `/plugins/{name}` | Start a new signal processing job (Multipart). |
| `GET` | `/plugins/{job_id}/status` | Poll the status and progress of a job. |
| `GET` | `/plugins/{job_id}/result` | Retrieve the final output of a completed job. |

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
iqengine-plugin = { git = "https://github.com/loic-fejoz/iqengine-plugin-rs/", branch = "main" }
```

## Usage Example

### 1. Implement your Plugin

Define your custom parameters and implement the `IQFunction` trait.

```rust
use iqengine_plugin::server::{IQFunction, FunctionParameters, FunctionParamsBuilder, Output, JobStore, IQEngineError, FunctionPostRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MyParams {
    pub gain: f32,
}

pub struct MyPlugin {}

impl IQFunction<MyParams> for MyPlugin {
    fn parameters(&self) -> FunctionParameters {
        FunctionParamsBuilder::new()
            .custom_param("gain", "Amplitude gain", iqengine_plugin::server::CustomParamType::Number, Some("1.0"))
            .build()
    }

    async fn apply(
        &self,
        request: FunctionPostRequest<MyParams>,
        samples: Vec<num_complex::Complex32>,
        job_id: String,
        job_store: Arc<JobStore>,
    ) -> Result<Output, IQEngineError> {
        // Your DSP logic here
        let mut output = Output::new();
        // ...
        Ok(output)
    }
}
```

### 2. Run the Server

Use the `PluginServer` and `configure_plugin` helpers to set up the Actix-web server.

```rust
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let job_store = Arc::new(JobStore::new("jobs").unwrap());
    let orchestrator = Arc::new(Orchestrator::new(job_store));

    let mut plugin_server = PluginServer::new(orchestrator.clone());
    plugin_server.add_plugin::<MyPlugin, MyParams>("my-plugin");

    HttpServer::new(move || {
        let ps = plugin_server.clone();
        App::new()
            .configure(|cfg| ps.configure(cfg))
            .configure(|cfg| {
                configure_plugin::<MyPlugin, MyParams>(cfg, "my-plugin", Arc::new(MyPlugin {}));
            })
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
```

## Development

### Prerequisites
- Rust 1.75+
- `cargo`

### Testing
```bash
cargo check
cargo test
```

## License

Apache-2.0
