#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod model;
mod db;
mod log;

fn main() {
    tauri::Builder::default()
        // .manage(model::Model::default())
        .setup(|app| {
            model::Model::init(app)?;
            db::Database::init(app)?;
            // log::setup_logger(app, log::LoggerOutput::Stdout, tracing::Level::INFO);
            // log to ~/.config/ai.lbk.assistant/logs
            log::setup_logger(app, log::LoggerOutput::default(), tracing::Level::INFO);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            model::logic::load_model_config,
            model::logic::load_dynamic_model,
            model::logic::unload_dynamic_model,
            model::logic::predict,
            model::simulated::load_model_config_simulated,
            model::simulated::load_dynamic_model_simulated,
            model::simulated::unload_dynamic_model_simulated,
            model::simulated::predict_simulated,
            db::logic::connect,
            db::logic::add_model_config,
            db::logic::get_model_configs,
            db::logic::delete_model_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
