use std::path::PathBuf;

use chrono::Local;
use futures::StreamExt;
use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use strum::EnumString;
use tauri_sys::{
    dialog,
    event::listen,
    tauri::{self, invoke},
};

pub mod components;
pub mod pages;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PayloadModelParams {
    pub params: ModelParams,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModelParams {
    pub model_params: ModelParameters,
}



#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ModelParameters {
    pub prefer_mmap: bool,
    pub context_size: usize,
    pub lora_adapters: Option<Vec<PathBuf>>,
    pub use_gpu: bool,
    pub gpu_layers: Option<usize>,
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            prefer_mmap: true,
            context_size: 2048,
            lora_adapters: None,
            use_gpu: false,
            gpu_layers: None,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum Entity {
    #[default]
    User,
    Bot,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Message {
    pub content: String,
    pub entity: Entity,
    pub is_loading: bool,
    pub time: String,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            content: Default::default(),
            entity: Default::default(),
            is_loading: Default::default(),
            time: Local::now().format("%a %e %b %Y, %T").to_string(),
        }
    }
}

impl Message {
    pub fn update_content(&mut self, content: &str) {
        self.content.push_str(content);
    }
    pub fn done(&mut self) {
        self.is_loading = false;
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Payload {
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PayloadId {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PayloadModelConfig {
    #[serde(rename(serialize = "modelConfig"))]
    pub model_config: ModelConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Default, Eq, Hash, PartialEq, Copy)]
pub enum ModelArchitecture {
    Bloom,
    Gpt2,
    GptJ,
    GptNeoX,
    #[default]
    Llama,
    Mpt,
    Falcon,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Default, PartialEq)]
pub enum TokenizerSource {
    #[default]
    Embedded,
    HuggingFaceTokenizerFile(PathBuf),
    HuggingFaceRemote(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ModelConfig {
    pub name: String,
    pub model_architecture: ModelArchitecture,
    pub model_path: PathBuf,
    pub tokenizer_source: TokenizerSource,
}

impl ModelConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Clone, Debug)]
pub struct ModelConfigState(bool);

#[derive(Deserialize)]
pub struct InferenceStats {
    /// How long it took to feed the prompt.
    pub feed_prompt_duration: std::time::Duration,
    /// How many tokens the prompt was.
    pub prompt_tokens: usize,
    /// How long it took to predict new tokens.
    pub predict_duration: std::time::Duration,
    /// The number of predicted tokens.
    pub predict_tokens: usize,
}
impl std::fmt::Display for InferenceStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Self {
            feed_prompt_duration,
            prompt_tokens,
            predict_duration,
            predict_tokens,
        } = *self;

        let feed_prompt_duration = feed_prompt_duration.as_millis();
        let predict_duration = predict_duration.as_millis();
        let per_token_duration = if predict_tokens == 0 {
            0.0
        } else {
            predict_duration as f64 / predict_tokens as f64
        };

        writeln!(f, "feed_prompt_duration: {}ms", feed_prompt_duration)?;
        writeln!(f, "prompt_tokens: {}", prompt_tokens)?;
        writeln!(f, "predict_duration: {}ms", predict_duration)?;
        writeln!(f, "predict_tokens: {}", predict_tokens)?;
        write!(f, "per_token_duration: {:.3}ms", per_token_duration)
    }
}

pub fn setup(cx: Scope) {
    let (model_config_loaded, set_model_config_loaded) =
        create_signal(cx, ModelConfigState::default());
    provide_context(cx, (model_config_loaded, set_model_config_loaded));
    let (messages, set_messages) = create_signal(cx, Vec::<Message>::new());
    provide_context(cx, (messages, set_messages));
    let (is_model_connected, set_is_model_connected) = create_signal(cx, false);
    provide_context(cx, (is_model_connected, set_is_model_connected));
    let (model_params, set_model_params) = create_signal(cx, ModelParameters::default());
    provide_context(cx, (model_params, set_model_params));
    let (model_configs, set_model_configs) = create_signal(cx, Vec::<ModelConfig>::new());
    // let (models, set_models) = create_signal(cx, vec![ModelConfig::default(); 10]);
    provide_context(cx, (model_configs, set_model_configs));

    // Start listening
    spawn_local(async move {
        warn!("Start listening");
        match listen::<String>("predict_event").await {
            Ok(mut events) => {
                while let Some(event) = events.next().await {
                    set_messages.update(|messages| {
                        messages
                            .last_mut()
                            .unwrap()
                            .update_content(event.payload.as_ref())
                    });
                }
                debug_warn!("Stopped listening");
                warn!("Stopped listening");
            }
            Err(err) => {
                error!("Listen external got an error: {err}")
            }
        }
    });

    // Init the database listening
    spawn_local(async move {
        log!("Init the database");
        match invoke::<(), String>("connect", &()).await {
            Ok(msg) => log!("DB init {msg}"),
            Err(err) => log!("DB init {err:#?}"),
        }
    });
    // Listen for database changes
    spawn_local(async move {
        log!("Database sync");
        match listen::<()>("db_sync_event").await {
            Ok(mut events) => {
                while events.next().await.is_some() {
                    match tauri::invoke::<_, Vec<ModelConfig>>("get_model_configs", &()).await {
                        Ok(model_configs) => {
                            set_model_configs(model_configs);
                        }
                        Err(err) => {
                            match dialog::MessageDialogBuilder::new()
                                .set_title("Get model configs (Sync)")
                                .set_kind(dialog::MessageDialogKind::Error)
                                .message(format!("{err:#?}").as_str())
                                .await
                            {
                                Ok(()) => (),
                                Err(err) => error!("Dialog error (sync) get model configs: {err}"),
                            };
                        }
                    };
                }
                debug_warn!("Stopped listening");
                warn!("Stopped listening");
            }
            Err(err) => {
                error!("Listen external got an error: {err}")
            }
        }
    });

    // Get local model configs
    spawn_local(async move {
        match tauri::invoke::<_, Vec<ModelConfig>>("get_model_configs", &()).await {
            Ok(model_configs) => {
                set_model_configs(model_configs);
            }
            Err(err) => {
                match dialog::MessageDialogBuilder::new()
                    .set_title("Get model configs")
                    .set_kind(dialog::MessageDialogKind::Error)
                    .message(format!("{err:#?}").as_str())
                    .await
                {
                    Ok(()) => (),
                    Err(err) => error!("Dialog error get model configs: {err}"),
                };
            }
        };
    });

    provide_meta_context(cx);
}
