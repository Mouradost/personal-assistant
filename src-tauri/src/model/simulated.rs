use tauri::{Runtime, Window};

use super::{Model, ModelConfig};

#[tauri::command]
pub async fn predict_simulated<R: Runtime>(
    _win: Window<R>,
    _message: &str,
    _state: tauri::State<'_, Model>,
) -> Result<llm::InferenceStats, String> {
    Ok(llm::InferenceStats::default())
}

#[tauri::command]
pub async fn load_dynamic_model_simulated(
    _state: tauri::State<'_, Model>,
) -> Result<String, String> {
    Ok("".to_owned())
}

#[tauri::command]
pub async fn unload_dynamic_model_simulated(_state: tauri::State<'_, Model>) -> Result<String, String> {
    Ok("".to_owned())
}

#[tauri::command]
pub async fn load_model_config_simulated(
    _model_config: ModelConfig,
    _state: tauri::State<'_, Model>,
) -> Result<String, String> {
    Ok("".to_owned())
}
