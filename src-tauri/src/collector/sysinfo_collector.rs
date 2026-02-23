use std::collections::HashMap;
use sysinfo::{
    Process, ProcessStatus, System, Users,
};
use tracing::trace;

use crate::models::{ProcessDto, ProcessStatus as DtoStatus};

/// Wraps `sysinfo::System` and provides collected process snapshots.
pub struct SysinfoCollector {
    system: System,
    users: Users,
}

impl SysinfoCollector {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let users = Users::new_with_refreshed_list();
        Self { system, users }
    }

    /// Refresh process list and return a map of PID → DTO.
    pub fn collect(&mut self) -> HashMap<u32, ProcessDto> {
        self.system.refresh_all();
        self.users.refresh();

        let current_user = Self::get_current_username();

        let mut result = HashMap::new();

        for (pid, process) in self.system.processes() {
            let pid_u32 = pid.as_u32();
            let dto = self.map_process(pid_u32, process, &current_user);
            result.insert(pid_u32, dto);
        }

        trace!("Collected {} processes", result.len());
        result
    }

    fn map_process(&self, pid: u32, p: &Process, current_user: &str) -> ProcessDto {
        let status = map_status(p.status());

        let user_id = p.user_id();
        let user_name: Option<String> = user_id.and_then(|uid| {
            self.users
                .iter()
                .find(|u| u.id() == uid)
                .map(|u| u.name().to_string())
        });

        let path = p.exe().map(|e| e.to_string_lossy().to_string());

        let parent_pid = p.parent().map(|pp| pp.as_u32());

        let needs_elevation = user_name
            .as_deref()
            .map(|u| u != current_user && !u.is_empty())
            .unwrap_or(false);

        let cmd: Vec<String> = p.cmd().iter().map(|s| s.to_string_lossy().to_string()).collect();

        ProcessDto {
            pid,
            name: p.name().to_string_lossy().to_string(),
            status,
            cpu_percent: p.cpu_usage(),
            memory_bytes: p.memory(),
            user: user_name,
            path,
            parent_pid,
            start_time: Some(p.start_time()),
            needs_elevation,
            cmd,
        }
    }

    pub fn get_current_username() -> String {
        #[cfg(unix)]
        {
            std::env::var("USER")
                .or_else(|_| std::env::var("LOGNAME"))
                .unwrap_or_else(|_| "unknown".to_string())
        }
        #[cfg(windows)]
        {
            std::env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string())
        }
    }
}

fn map_status(s: ProcessStatus) -> DtoStatus {
    match s {
        ProcessStatus::Run => DtoStatus::Running,
        ProcessStatus::Sleep => DtoStatus::Sleeping,
        ProcessStatus::Stop => DtoStatus::Stopped,
        ProcessStatus::Zombie => DtoStatus::Zombie,
        _ => DtoStatus::Unknown,
    }
}

impl Default for SysinfoCollector {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_returns_processes() {
        let mut collector = SysinfoCollector::new();
        let procs = collector.collect();
        // Should always have at least the current process
        assert!(!procs.is_empty(), "Expected at least one process");
    }

    #[test]
    fn test_current_pid_present() {
        let mut collector = SysinfoCollector::new();
        let procs = collector.collect();
        let current_pid = std::process::id();
        assert!(
            procs.contains_key(&current_pid),
            "Current PID {} should be in the collected list",
            current_pid
        );
    }
}
