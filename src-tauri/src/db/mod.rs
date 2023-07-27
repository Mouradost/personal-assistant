use std::sync::Arc;
use surrealdb::{
    // engine::local::{Db, File},
    engine::local::{Db, Mem},
    Surreal,
};
use tauri::async_runtime::Mutex;
use tauri::{App, Manager};

pub mod logic;

#[derive(Default)]
pub struct Database {
    pub db: Arc<Mutex<Option<Surreal<Db>>>>,
}

impl Database {
    pub fn init(app: &App) -> Result<(), String> {
        let app_handle = app.app_handle();
        tauri::async_runtime::spawn(async move {
            let app_data_dir = app_handle
                .path_resolver()
                .app_data_dir()
                .unwrap_or_default();
            tracing::info!("App data dir is: {}", app_data_dir.display());
            // Initialize the database in the app data file
            // let db = Surreal::new::<File>(format!("{}/db", app_data_dir.display()).as_str())
            let db = Surreal::new::<Mem>(()).await.unwrap();
            // Select a specific namespace / database
            db.use_ns("my_ns")
                .use_db("my_db")
                .await
                .map_err(|err| err.to_string())
                .unwrap();
            app_handle.manage(Database {
                db: Arc::new(Mutex::new(Some(db))),
            });
            tracing::info!("DB setup succesful");
        });
        Ok(())
    }
}
