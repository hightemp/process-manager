use std::sync::Arc;

use parking_lot::Mutex;
use tauri::State;
use tracing::debug;

use crate::{
    error::AppError,
    models::{apply_sort, ProcessDetails, ProcessDto, ProcessFilter, SortSpec},
    state::AppState,
};

type SharedState = Arc<Mutex<AppState>>;

/// Return all processes (filtered and sorted server-side).
#[tauri::command]
pub fn list_processes(
    filter: Option<ProcessFilter>,
    sort: Option<SortSpec>,
    state: State<'_, SharedState>,
) -> Result<Vec<ProcessDto>, AppError> {
    let s = state.lock();
    let current_user = s.current_user.clone();
    let filter = filter.unwrap_or_default();

    let mut result: Vec<ProcessDto> = s
        .snapshot
        .values()
        .filter(|p| filter.matches(p, &current_user))
        .cloned()
        .collect();

    if let Some(sort_spec) = sort {
        apply_sort(&mut result, &sort_spec);
    } else {
        // Default: sort by CPU descending
        result.sort_by(|a, b| {
            b.cpu_percent
                .partial_cmp(&a.cpu_percent)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    debug!("list_processes: returning {} entries", result.len());
    Ok(result)
}

/// Return detailed information for a single process.
#[tauri::command]
pub fn process_details(
    pid: u32,
    state: State<'_, SharedState>,
) -> Result<ProcessDetails, AppError> {
    let s = state.lock();
    let dto = s
        .snapshot
        .get(&pid)
        .cloned()
        .ok_or(AppError::NotFound { pid })?;

    Ok(ProcessDetails {
        dto,
        threads: None,          // sysinfo does not expose this easily
        virtual_memory_bytes: None,
        disk_read_bytes: None,
        disk_written_bytes: None,
        open_files_count: None,
        environment: None,
    })
}

/// Update the auto-refresh interval (200 – 10_000 ms).
#[tauri::command]
pub fn set_refresh_interval(
    ms: u64,
    state: State<'_, SharedState>,
) -> Result<(), AppError> {
    let ms = ms.clamp(200, 10_000);
    state.lock().refresh_interval_ms = ms;
    debug!("Refresh interval set to {}ms", ms);
    Ok(())
}

/// Pause or resume auto-refresh.
#[tauri::command]
pub fn set_paused(paused: bool, state: State<'_, SharedState>) -> Result<(), AppError> {
    state.lock().paused = paused;
    debug!("Auto-refresh paused: {}", paused);
    Ok(())
}
