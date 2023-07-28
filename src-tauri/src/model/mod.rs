use std::{
    path::PathBuf,
    sync::{Arc, Mutex}
};
use serde::{Deserialize, Serialize};
use tauri::{App, Manager};

pub mod logic;
pub mod simulated;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(remote="llm::ModelParameters")]
pub struct ModelParameters {
    pub prefer_mmap: bool,
    pub context_size: usize,
    pub lora_adapters: Option<Vec<PathBuf>>,
    pub use_gpu: bool,
    pub gpu_layers: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct ModelParametersWrapper{
    #[serde(with = "ModelParameters")]
    model_params: llm::ModelParameters
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(remote="llm::ModelArchitecture")]
pub enum ModelArchitecture {
    Bloom,
    Gpt2,
    GptJ,
    GptNeoX,
    #[default]
    Llama,
    Mpt,
    // Falcon
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(remote="llm::TokenizerSource")]
pub enum TokenizerSource {
    #[default]
    Embedded,
    HuggingFaceTokenizerFile(PathBuf),
    HuggingFaceRemote(String),
    HuggingFaceTokenizerString(String),
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ModelConfig {
    pub name: String,
    #[serde(with = "ModelArchitecture")]
    pub model_architecture: llm::ModelArchitecture,
    pub model_path: PathBuf,
    #[serde(with = "TokenizerSource")]
    pub tokenizer_source: llm::TokenizerSource,
}


impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            model_architecture: llm::ModelArchitecture::Llama,
            model_path: PathBuf::default(),
            tokenizer_source: llm::TokenizerSource::Embedded,
        }
    }
}

#[derive(Default)]
pub struct Model {
    model: Arc<Mutex<Option<Box<dyn llm::Model>>>>,
    model_config: Arc<Mutex<Option<ModelConfig>>>,
}

impl Model {
    pub fn init(app: &App) -> Result<(), String>{
        // app.manage(Model{
        //     model_config: Arc::new(Mutex::new(Some(ModelConfig::default()))),
        //     ..Default::default()
        // });
        app.manage(Model::default());
        Ok(())
    }
    
}

