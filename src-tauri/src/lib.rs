pub mod collector;
pub mod commands;
pub mod error;
pub mod models;
pub mod state;
pub mod updater;

use std::sync::Arc;

use parking_lot::Mutex;
use tracing::info;
use tracing_subscriber::EnvFilter;

use collector::SysinfoCollector;
use commands::*;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialise structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    info!("Process Manager starting up");

    let current_user = SysinfoCollector::get_current_username();
    let app_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::new(current_user)));
    let state_for_updater = Arc::clone(&app_state);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(app_state)
        .setup(|app| {
            let handle = app.handle().clone();
            updater::start_updater(handle, state_for_updater);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_processes,
            process_details,
            set_refresh_interval,
            set_paused,
            kill_process,
            open_path,
            copy_to_clipboard,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
