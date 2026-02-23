use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use parking_lot::Mutex;
use tauri::{AppHandle, Emitter};
use tokio::time;
use tracing::{error, info};

// [FIX] Use tauri::async_runtime::spawn instead of tokio::spawn.
// tokio::spawn requires an active Tokio reactor, but Tauri's .setup() callback
// runs before one is available. tauri::async_runtime::spawn dispatches onto
// Tauri's own managed runtime, which is always ready during setup.

use crate::{
    collector::SysinfoCollector,
    models::{ProcessDto, ProcessUpdateEvent},
    state::AppState,
};

pub const EVENT_PROCESSES_UPDATE: &str = "processes:update";
pub const EVENT_PROCESS_GONE: &str = "process:gone";

/// Starts the background refresh loop in a Tokio task.
/// Interval is read from AppState on each tick to support live changes.
pub fn start_updater(app_handle: AppHandle, state: Arc<Mutex<AppState>>) {
    tauri::async_runtime::spawn(async move {
        let mut collector = SysinfoCollector::new();

        // Initial snapshot
        let initial = collector.collect();
        {
            let mut s = state.lock();
            s.snapshot = initial;
        }

        info!("Updater started");

        loop {
            // Read interval + paused flag
            let (interval_ms, paused) = {
                let s = state.lock();
                (s.refresh_interval_ms, s.paused)
            };

            time::sleep(Duration::from_millis(interval_ms)).await;

            if paused {
                continue;
            }

            let new_snapshot = collector.collect();
            let event = {
                let mut s = state.lock();
                let event = diff_snapshots(&s.snapshot, &new_snapshot);
                s.snapshot = new_snapshot;
                event
            };

            // Only emit if something changed
            if !event.added.is_empty() || !event.updated.is_empty() || !event.removed.is_empty() {
                if let Err(e) = app_handle.emit(EVENT_PROCESSES_UPDATE, &event) {
                    error!("Failed to emit processes:update: {}", e);
                }
            }
        }
    });
}

/// Compute diff between old and new snapshots.
fn diff_snapshots(
    old: &HashMap<u32, ProcessDto>,
    new: &HashMap<u32, ProcessDto>,
) -> ProcessUpdateEvent {
    let mut added = Vec::new();
    let mut updated = Vec::new();
    let mut removed = Vec::new();

    // Find added + updated
    for (pid, new_proc) in new {
        match old.get(pid) {
            None => added.push(new_proc.clone()),
            Some(old_proc) => {
                if is_changed(old_proc, new_proc) {
                    updated.push(new_proc.clone());
                }
            }
        }
    }

    // Find removed
    for pid in old.keys() {
        if !new.contains_key(pid) {
            removed.push(*pid);
        }
    }

    let timestamp_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    ProcessUpdateEvent {
        added,
        updated,
        removed,
        timestamp_ms,
    }
}

/// Check if relevant fields changed enough to warrant an update notification.
fn is_changed(old: &ProcessDto, new: &ProcessDto) -> bool {
    // Use a small epsilon for floating-point comparison
    (old.cpu_percent - new.cpu_percent).abs() > 0.1
        || old.memory_bytes != new.memory_bytes
        || old.status != new.status
}
