use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use serde::Serialize;
use uuid::Uuid;
use super::{Orchestrator, IQFunction, FunctionPostRequest, MetadataFile, DataType};

#[derive(Clone)]
pub struct PluginServer {
    pub orchestrator: Arc<Orchestrator>,
    pub function_names: Vec<String>,
}

impl PluginServer {
    pub fn new(orchestrator: Arc<Orchestrator>) -> Self {
        Self {
            orchestrator,
            function_names: Vec::new(),
        }
    }

    pub fn add_plugin<F, P>(&mut self, name: &str) 
    where
        P: Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
        F: IQFunction<P> + Send + Sync + 'static,
    {
        self.function_names.push(name.to_string());
    }

    pub fn configure(self, cfg: &mut web::ServiceConfig) {
        let function_names = web::Data::new(self.function_names.clone());
        let orchestrator = web::Data::new(self.orchestrator.clone());

        cfg.app_data(function_names)
           .app_data(orchestrator)
           .route("/plugins", web::get().to(get_plugins_handler))
           .service(
               web::resource("/plugins/{job_id}/status")
                   .route(web::get().to(get_job_status_handler)),
           )
           .service(
               web::resource("/plugins/{job_id}/result")
                   .route(web::get().to(get_job_result_handler)),
           );
    }
}

pub fn configure_plugin<F, P>(
    cfg: &mut web::ServiceConfig,
    function_name: &str,
    plugin: Arc<F>,
) where
    P: Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
    F: IQFunction<P> + Send + Sync + 'static,
{
    let function_name = function_name.to_string();
    cfg.service(
        web::resource(&format!("/plugins/{}", function_name))
            .app_data(web::Data::new(plugin))
            .app_data(web::Data::new(function_name))
            .route(web::get().to(get_plugin_definition_handler::<F, P>))
            .route(web::post().to(start_plugin_handler::<F, P>)),
    );
}

pub async fn get_plugins_handler(
    function_names: web::Data<Vec<String>>,
) -> impl Responder {
    HttpResponse::Ok().json(function_names.get_ref())
}

pub async fn get_job_status_handler(
    orchestrator: web::Data<Arc<Orchestrator>>,
    path: web::Path<String>,
) -> impl Responder {
    let job_id = path.into_inner();
    match orchestrator.get_status(&job_id) {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_job_result_handler(
    orchestrator: web::Data<Arc<Orchestrator>>,
    path: web::Path<String>,
) -> impl Responder {
    let job_id = path.into_inner();
    match orchestrator.get_result(&job_id) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_plugin_definition_handler<F, P>(
    plugin: web::Data<Arc<F>>,
) -> impl Responder
where
    P: Serialize + Send + Sync + 'static,
    F: IQFunction<P> + Send + Sync + 'static,
{
    HttpResponse::Ok().json(plugin.parameters())
}

pub async fn start_plugin_handler<F, P>(
    orchestrator: web::Data<Arc<Orchestrator>>,
    plugin: web::Data<Arc<F>>,
    function_name: web::Data<String>,
    mut payload: Multipart,
) -> impl Responder
where
    P: Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
    F: IQFunction<P> + Send + Sync + 'static,
{
    let mut iq_data = Vec::new();
    let mut metadata_file: Option<MetadataFile> = None;
    let mut custom_params: Option<P> = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let name = field.name().to_string();

        match name.as_str() {
            "iq_file" => {
                while let Some(chunk) = field.next().await {
                    iq_data.extend_from_slice(&chunk.unwrap());
                }
            }
            "metadata_file" => {
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    bytes.extend_from_slice(&chunk.unwrap());
                }
                metadata_file = serde_json::from_slice(&bytes).ok();
            }
            "custom_params" => {
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    bytes.extend_from_slice(&chunk.unwrap());
                }
                custom_params = serde_json::from_slice(&bytes).ok();
            }
            _ => {}
        }
    }

    let metadata = match metadata_file {
        Some(m) => m,
        None => return HttpResponse::BadRequest().body("Missing metadata_file"),
    };

    let samples = match metadata.data_type {
        DataType::IqSlashCf32Le => {
            iq_data.chunks_exact(8)
                .map(|c| {
                    let re = f32::from_le_bytes(c[0..4].try_into().unwrap());
                    let im = f32::from_le_bytes(c[4..8].try_into().unwrap());
                    num_complex::Complex32::new(re, im)
                })
                .collect()
        }
        _ => return HttpResponse::BadRequest().body("Unsupported data type"),
    };

    let request = FunctionPostRequest::new(metadata.clone(), custom_params);
    let job_id = Uuid::new_v4().to_string();

    match orchestrator.start_job(
        job_id,
        function_name.get_ref().clone(),
        metadata.file_name,
        plugin.get_ref().clone(),
        request,
        samples,
    ).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
