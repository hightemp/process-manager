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

#[cfg(all(debug_assertions, target_os = "linux"))]
fn ensure_localhost_no_proxy() {
    const LOCAL_TARGETS: [&str; 3] = ["localhost", "127.0.0.1", "::1"];

    fn merge_no_proxy_var(name: &str) {
        let existing = std::env::var(name).unwrap_or_default();
        let mut parts: Vec<String> = existing
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .collect();

        for target in LOCAL_TARGETS {
            if !parts.iter().any(|p| p.eq_ignore_ascii_case(target)) {
                parts.push(target.to_string());
            }
        }

        let merged = parts.join(",");
        std::env::set_var(name, merged);
    }

    merge_no_proxy_var("NO_PROXY");
    merge_no_proxy_var("no_proxy");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(all(debug_assertions, target_os = "linux"))]
    ensure_localhost_no_proxy();

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
