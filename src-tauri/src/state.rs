use std::collections::HashMap;

use crate::models::ProcessDto;

/// Shared application state — kept behind an `Arc<Mutex<AppState>>` in Tauri.
pub struct AppState {
    /// Latest process snapshot (PID → DTO).
    pub snapshot: HashMap<u32, ProcessDto>,
    /// Refresh interval in milliseconds.
    pub refresh_interval_ms: u64,
    /// Whether auto-refresh is paused.
    pub paused: bool,
    /// Current OS username (cached at startup).
    pub current_user: String,
}

impl AppState {
    pub fn new(current_user: String) -> Self {
        Self {
            snapshot: HashMap::new(),
            refresh_interval_ms: 1000,
            paused: false,
            current_user,
        }
    }
}
