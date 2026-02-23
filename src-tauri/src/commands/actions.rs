use std::sync::Arc;

use parking_lot::Mutex;
use tauri::State;
use tracing::info;

use crate::{
    error::AppError,
    models::KillMode,
    state::AppState,
};

type SharedState = Arc<Mutex<AppState>>;

// ---------------------------------------------------------------------------
// Kill / Terminate
// ---------------------------------------------------------------------------

/// Send a termination signal to a process.
/// `mode = "terminate"` → SIGTERM / graceful
/// `mode = "kill"`      → SIGKILL / forceful
#[tauri::command]
pub fn kill_process(
    pid: u32,
    mode: KillMode,
    state: State<'_, SharedState>,
) -> Result<(), AppError> {
    // Validate PID is known
    {
        let s = state.lock();
        if !s.snapshot.contains_key(&pid) {
            return Err(AppError::NotFound { pid });
        }
    }

    info!("kill_process: pid={} mode={:?}", pid, mode);

    #[cfg(unix)]
    {
        kill_unix(pid, &mode)
    }
    #[cfg(windows)]
    {
        kill_windows(pid, &mode)
    }
    #[cfg(not(any(unix, windows)))]
    {
        Err(AppError::Unsupported {
            feature: "kill_process".to_string(),
        })
    }
}

// ---------------------------------------------------------------------------
// Unix implementation
// ---------------------------------------------------------------------------

#[cfg(unix)]
fn kill_unix(pid: u32, mode: &KillMode) -> Result<(), AppError> {
    use std::io;

    let sig = match mode {
        KillMode::Terminate => libc::SIGTERM,
        KillMode::Kill => libc::SIGKILL,
    };

    let ret = unsafe { libc::kill(pid as libc::pid_t, sig) };

    if ret == 0 {
        Ok(())
    } else {
        let err = io::Error::last_os_error();
        let raw = err.raw_os_error().unwrap_or(0);
        if raw == libc::EPERM {
            Err(AppError::PermissionDenied {
                pid,
                message: "Insufficient permissions to signal this process".to_string(),
            })
        } else if raw == libc::ESRCH {
            Err(AppError::NotFound { pid })
        } else {
            Err(AppError::OsError {
                message: err.to_string(),
            })
        }
    }
}

// ---------------------------------------------------------------------------
// Windows implementation
// ---------------------------------------------------------------------------

#[cfg(windows)]
fn kill_windows(pid: u32, _mode: &KillMode) -> Result<(), AppError> {
    // On Windows both "terminate" and "kill" use TerminateProcess.
    // A graceful alternative (WM_CLOSE) is complex and not guaranteed — use TerminateProcess.
    use windows::Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE},
    };

    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, false, pid).map_err(|e| {
            AppError::PermissionDenied {
                pid,
                message: e.to_string(),
            }
        })?;

        let result = TerminateProcess(handle, 1);
        CloseHandle(handle).ok();

        result.map_err(|e| {
            if e.code().0 as u32 == 0x80070005 {
                // Access denied
                AppError::PermissionDenied {
                    pid,
                    message: e.to_string(),
                }
            } else {
                AppError::OsError {
                    message: e.to_string(),
                }
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Open file location
// ---------------------------------------------------------------------------

/// Open the directory containing the process executable in the OS file manager.
#[tauri::command]
pub async fn open_path(
    pid: u32,
    state: State<'_, SharedState>,
) -> Result<(), AppError> {
    let path = {
        let s = state.lock();
        s.snapshot
            .get(&pid)
            .and_then(|p| p.path.clone())
            .ok_or(AppError::NotFound { pid })?
    };

    let dir = std::path::Path::new(&path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or(path);

    tauri_plugin_opener::open_path(dir, None::<&str>)
        .map_err(|e| AppError::OsError { message: e.to_string() })
}

// ---------------------------------------------------------------------------
// Clipboard
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn copy_to_clipboard(text: String, app_handle: tauri::AppHandle) -> Result<(), AppError> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app_handle
        .clipboard()
        .write_text(text)
        .map_err(|e| AppError::OsError { message: e.to_string() })
}

// ---------------------------------------------------------------------------
// Libc dependency — only needed on Unix (already transitively via sysinfo on Linux/macOS)
// ---------------------------------------------------------------------------

#[cfg(unix)]
extern crate libc;
