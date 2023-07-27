use std::{convert::Infallible, io::Write};
use tauri::{Runtime, Window};

use super::{Model, ModelConfig};


#[tauri::command]
pub async fn predict<R: Runtime>(
    win: Window<R>,
    message: &str,
    state: tauri::State<'_, Model>,
) -> Result<llm::InferenceStats, String> {
    tracing::debug!("Predict {message:#?}");
    let model_guard = state.model.lock().map_err(|err| err.to_string())?;
    let model = match model_guard.as_ref(){
        Some(model) => model,
        None => return Err("Model not found".to_string()),
    };
    let mut session = model.start_session(Default::default());

    let res = session.infer::<Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: message.into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        // OutputRequest
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::PromptToken(t) => {
                print!("{t}");
                std::io::stdout().flush().unwrap();
                Ok(llm::InferenceFeedback::Continue)
                // Ok(llm::InferenceFeedback::Halt)
            }
            llm::InferenceResponse::InferredToken(t) => {
                print!("{t}");
                std::io::stdout().flush().unwrap();
                let _ = win
                    .emit("predict_event", t.as_str())
                    .map_err(|err| err.to_string());
                Ok(llm::InferenceFeedback::Continue)
                // Ok(llm::InferenceFeedback::Halt)
            }
            llm::InferenceResponse::EotToken => Ok(llm::InferenceFeedback::Halt),
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    match res {
        Ok(result) => {
            tracing::debug!("\n{result}");
            Ok(result)
        }
        Err(err) => {
            tracing::error!("\n{err}");
            Err("Error".to_string())
        }
    }
}

#[tauri::command]
pub async fn load_dynamic_model(
    state: tauri::State<'_, Model>,
) -> Result<String, String> {
    tracing::debug!("Loading model");
    let model_config_guard = state.model_config.lock().map_err(|err| err.to_string())?;
    let model_config = match model_config_guard.as_ref(){
        Some(model_config) => model_config,
        None => return Err("Model config not found".to_string()),
    };

    tracing::info!("Got model_config: {:#?}", state.model_config);
    let model = llm::load_dynamic(
        Some(model_config.model_architecture),
        &model_config.model_path,
        model_config.tokenizer_source.clone(),
        Default::default(),
        llm::load_progress_callback_stdout,
    )
    .map_err(|err| err.to_string())?;
    *state.model.lock().map_err(|err| err.to_string())? = Some(model);

    Ok(format!("Model loaded"))
}

#[tauri::command]
pub async fn unload_dynamic_model(state: tauri::State<'_, Model>) -> Result<String, String> {
    tracing::info!("Unloading model");
    *state.model.lock().map_err(|err| err.to_string())? = None;
    Ok(String::from("Model unloaded"))
}

#[tauri::command]
pub async fn load_model_config(
    model_config: ModelConfig,
    state: tauri::State<'_, Model>,
) -> Result<String, String> {
    tracing::info!("Loading config {model_config:#?}");
    *state.model_config.lock().map_err(|err| err.to_string())? =  Some(model_config);
    Ok(format!("Model config loaded"))
}

