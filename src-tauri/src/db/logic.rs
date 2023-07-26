use crate::model::ModelConfig;

use super::Database;
use surrealdb::{engine::local::File, Surreal};
use tauri::{Runtime, Window};

#[tauri::command]
pub async fn connect(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, Database>,
) -> Result<String, String> {
    // Check if there is a connection
    if state.db.lock().await.is_some() {
        return Ok("Database already connected".to_owned());
    };
    // Get the data directory
    let app_data_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap_or_default();
    tracing::info!("App data dir is: {}", app_data_dir.display());
    // Connect to the local storage
    let db = Surreal::new::<File>(format!("{}/db", app_data_dir.display()).as_str())
        .await
        .map_err(|err| err.to_string())?;
    // Select a specific namespace / database
    db.use_ns("my_ns")
        .use_db("my_db")
        .await
        .map_err(|err| err.to_string())?;
    // Set the database
    *state.db.lock().await = Some(db);
    Ok("Connected to the database".to_owned())
}

#[tauri::command]
pub async fn add_model_config<R: Runtime>(
    win: Window<R>,
    model_config: ModelConfig,
    state: tauri::State<'_, Database>,
) -> Result<String, String> {
    // Get the database
    let db = state.db.lock().await;
    // Check if it exists
    let db = match db.as_ref() {
        Some(db) => db,
        None => return Err("Database not connected, please reconnect to the database".to_string()),
    };
    // Create a new model_config
    let created: Option<ModelConfig> = db
        .create(("model_config", model_config.name.as_str()))
        .content(model_config)
        .await
        .map_err(|err| err.to_string())?;
    tracing::info!("Model config added: {:#?}", created);
    match created {
        Some(created) => {
            let _ = win
                .emit("db_sync_event", ())
                .map_err(|err| err.to_string());
            Ok(format!("Model config added: {created:#?}"))
        },
        None => Err("Model config already exists".to_string()),
    }

}

#[tauri::command]
pub async fn delete_model_config<R: Runtime>(
    win: Window<R>,
    name: String,
    state: tauri::State<'_, Database>,
) -> Result<String, String> {
    // Get the database
    let db = state.db.lock().await;
    // Check if it exists
    let db = match db.as_ref() {
        Some(db) => db,
        None => return Err("Database not connected, please reconnect to the database".to_string()),
    };
    // Create a new model_config
    let deleted: Option<ModelConfig> = db
        .delete(("model_config", name.as_str()))
        .await
        .map_err(|err| err.to_string())?;
    tracing::info!("Model config deleted: {:#?}", deleted);
    match deleted {
        Some(deleted) => {
            let _ = win
                .emit("db_sync_event", ())
                .map_err(|err| err.to_string());
            Ok(format!("Model config deleted: {deleted:#?}"))
        },
        None => Err("Model config not found".to_string()),
    }

}

#[tauri::command]
pub async fn get_model_configs(
    state: tauri::State<'_, Database>,
) -> Result<Vec<ModelConfig>, String> {
    // Get the database
    let db = state.db.lock().await;
    // Check if it exists
    let db = match db.as_ref() {
        Some(db) => db,
        None => return Err("Database not connected, please reconnect to the database".to_string()),
    };
    // Create a new model_config
    let model_configs: Vec<ModelConfig> = db
        .select("model_config")
        .await
        .map_err(|err| err.to_string())?;
    tracing::info!("Get all model configs {:#?}", model_configs);
    Ok(model_configs)
}
