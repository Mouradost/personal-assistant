use tauri::{App, Manager};
use tracing::Level;
use tracing_appender::rolling;

#[allow(unused)]
#[derive(Clone, Default)]
pub enum LoggerOutput {
    #[default]
    File,
    Stdout,
}

pub fn setup_logger(
    app: &App,
    logger_output: LoggerOutput,
    level: Level,
)
{
    let app_handle = app.app_handle();
    let subscribe_builder = tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .with_file(false)
        .with_thread_ids(false)
        .pretty(); // .compact()

    match logger_output {
        LoggerOutput::File => {
            let app_log_dir = app_handle.path_resolver().app_log_dir().unwrap_or_default();
            let log_file = rolling::daily(app_log_dir, "daily");
            subscribe_builder
                .with_writer(log_file)
                .with_ansi(false)
                .init();
        }
        LoggerOutput::Stdout => {
            subscribe_builder
                .with_ansi(true)
                .init();
        }
    }
}
