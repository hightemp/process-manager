use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Structured error type returned to the frontend as JSON.
#[derive(Debug, Error, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum AppError {
    #[error("Process {pid} not found")]
    NotFound { pid: u32 },

    #[error("Permission denied for process {pid}: {message}")]
    PermissionDenied { pid: u32, message: String },

    #[error("Invalid PID: {pid}")]
    InvalidPid { pid: u32 },

    #[error("OS error: {message}")]
    OsError { message: String },

    #[error("Feature not supported on this OS: {feature}")]
    Unsupported { feature: String },
}
