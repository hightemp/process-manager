use serde::{Deserialize, Serialize};

/// Process status mirrored from sysinfo.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProcessStatus {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Unknown,
}

/// Main DTO sent to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDto {
    pub pid: u32,
    pub name: String,
    pub status: ProcessStatus,
    /// Cumulative CPU percentage (0.0–100.0, per-core normalised by sysinfo)
    pub cpu_percent: f32,
    /// Resident set size in bytes
    pub memory_bytes: u64,
    /// OS username owning the process (None if unavailable / elevated needed)
    pub user: Option<String>,
    /// Full path to the executable (None for kernel threads or restricted)
    pub path: Option<String>,
    /// Parent process ID
    pub parent_pid: Option<u32>,
    /// Unix start timestamp (seconds); None if unavailable  
    pub start_time: Option<u64>,
    /// True when the process is owned by a different user or root
    pub needs_elevation: bool,
    /// Command-line arguments
    pub cmd: Vec<String>,
}

/// Filter applied server-side before returning results.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProcessFilter {
    /// Substring match against name, path or stringified PID
    pub search: Option<String>,
    /// Exact username match
    pub user: Option<String>,
    /// If true, only return processes owned by current user
    pub mine_only: Option<bool>,
    /// If true, return only system processes (pid < 500 heuristic + root owned)
    pub system_only: Option<bool>,
    /// If true, exclude system processes
    pub non_system_only: Option<bool>,
    /// Filter by status
    pub status: Option<ProcessStatus>,
    /// Minimum CPU threshold (inclusive)
    pub cpu_gt: Option<f32>,
    /// Minimum memory threshold in bytes (inclusive)
    pub memory_gt_bytes: Option<u64>,
}

/// Column names that can be used for sorting.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    Pid,
    Name,
    CpuPercent,
    MemoryBytes,
    User,
    Status,
    StartTime,
}

/// Sort order.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Full sort specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortSpec {
    pub field: SortField,
    pub direction: SortDirection,
}

/// Kill mode sent from the frontend.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum KillMode {
    /// Graceful termination (SIGTERM on Unix, TerminateProcess with timeout on Windows)
    Terminate,
    /// Forceful kill (SIGKILL on Unix, TerminateProcess immediate on Windows)
    Kill,
}

/// Incremental update event emitted by the updater task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessUpdateEvent {
    pub added: Vec<ProcessDto>,
    pub updated: Vec<ProcessDto>,
    pub removed: Vec<u32>,
    pub timestamp_ms: u64,
}

/// Detailed information for the detail panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDetails {
    pub dto: ProcessDto,
    pub threads: Option<u32>,
    pub virtual_memory_bytes: Option<u64>,
    pub disk_read_bytes: Option<u64>,
    pub disk_written_bytes: Option<u64>,
    pub open_files_count: Option<usize>,
    pub environment: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Filtering & sorting helpers (used by commands layer)
// ---------------------------------------------------------------------------

impl ProcessFilter {
    pub fn matches(&self, p: &ProcessDto, current_user: &str) -> bool {
        // Search filter
        if let Some(ref q) = self.search {
            let q_lower = q.to_lowercase();
            let pid_str = p.pid.to_string();
            let name_match = p.name.to_lowercase().contains(&q_lower);
            let path_match = p
                .path
                .as_deref()
                .map(|pa| pa.to_lowercase().contains(&q_lower))
                .unwrap_or(false);
            let pid_match = pid_str.contains(&q_lower);
            if !name_match && !path_match && !pid_match {
                return false;
            }
        }

        // User filter
        if let Some(ref u) = self.user {
            if p.user.as_deref() != Some(u.as_str()) {
                return false;
            }
        }

        if self.mine_only == Some(true) {
            if p.user.as_deref() != Some(current_user) {
                return false;
            }
        }

        if self.system_only == Some(true) && !is_system(p) {
            return false;
        }
        if self.non_system_only == Some(true) && is_system(p) {
            return false;
        }

        if let Some(ref st) = self.status {
            if &p.status != st {
                return false;
            }
        }

        if let Some(cpu_gt) = self.cpu_gt {
            if p.cpu_percent < cpu_gt {
                return false;
            }
        }

        if let Some(mem_gt) = self.memory_gt_bytes {
            if p.memory_bytes < mem_gt {
                return false;
            }
        }

        true
    }
}

fn is_system(p: &ProcessDto) -> bool {
    p.pid < 500
        || p.user
            .as_deref()
            .map(|u| matches!(u, "root" | "SYSTEM" | "NT AUTHORITY\\SYSTEM"))
            .unwrap_or(false)
}

pub fn apply_sort(processes: &mut Vec<ProcessDto>, sort: &SortSpec) {
    processes.sort_by(|a, b| {
        let ord = match sort.field {
            SortField::Pid => a.pid.cmp(&b.pid),
            SortField::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            SortField::CpuPercent => a
                .cpu_percent
                .partial_cmp(&b.cpu_percent)
                .unwrap_or(std::cmp::Ordering::Equal),
            SortField::MemoryBytes => a.memory_bytes.cmp(&b.memory_bytes),
            SortField::User => a
                .user
                .as_deref()
                .unwrap_or("")
                .cmp(b.user.as_deref().unwrap_or("")),
            SortField::Status => format!("{:?}", a.status).cmp(&format!("{:?}", b.status)),
            SortField::StartTime => a
                .start_time
                .unwrap_or(0)
                .cmp(&b.start_time.unwrap_or(0)),
        };
        if sort.direction == SortDirection::Desc {
            ord.reverse()
        } else {
            ord
        }
    });
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_proc(pid: u32, name: &str, cpu: f32, mem: u64, user: &str) -> ProcessDto {
        ProcessDto {
            pid,
            name: name.to_string(),
            status: ProcessStatus::Running,
            cpu_percent: cpu,
            memory_bytes: mem,
            user: Some(user.to_string()),
            path: Some(format!("/usr/bin/{}", name)),
            parent_pid: None,
            start_time: None,
            needs_elevation: false,
            cmd: vec![],
        }
    }

    #[test]
    fn test_filter_search_by_name() {
        let p = make_proc(1000, "chrome", 5.0, 1024, "alice");
        let f = ProcessFilter {
            search: Some("chro".to_string()),
            ..Default::default()
        };
        assert!(f.matches(&p, "alice"));
    }

    #[test]
    fn test_filter_search_by_pid() {
        let p = make_proc(1234, "firefox", 2.0, 512, "alice");
        let f = ProcessFilter {
            search: Some("1234".to_string()),
            ..Default::default()
        };
        assert!(f.matches(&p, "alice"));
    }

    #[test]
    fn test_filter_mine_only() {
        let p1 = make_proc(1, "init", 0.0, 100, "root");
        let p2 = make_proc(2, "bash", 0.0, 50, "alice");
        let f = ProcessFilter {
            mine_only: Some(true),
            ..Default::default()
        };
        assert!(!f.matches(&p1, "alice"));
        assert!(f.matches(&p2, "alice"));
    }

    #[test]
    fn test_filter_cpu_threshold() {
        let p1 = make_proc(10, "idle", 0.5, 100, "root");
        let p2 = make_proc(11, "compile", 90.0, 500, "alice");
        let f = ProcessFilter {
            cpu_gt: Some(50.0),
            ..Default::default()
        };
        assert!(!f.matches(&p1, "alice"));
        assert!(f.matches(&p2, "alice"));
    }

    #[test]
    fn test_sort_by_cpu_desc() {
        let mut procs = vec![
            make_proc(1, "a", 10.0, 100, "u"),
            make_proc(2, "b", 80.0, 100, "u"),
            make_proc(3, "c", 40.0, 100, "u"),
        ];
        apply_sort(
            &mut procs,
            &SortSpec {
                field: SortField::CpuPercent,
                direction: SortDirection::Desc,
            },
        );
        assert_eq!(procs[0].pid, 2);
        assert_eq!(procs[1].pid, 3);
        assert_eq!(procs[2].pid, 1);
    }

    #[test]
    fn test_sort_by_name_asc() {
        let mut procs = vec![
            make_proc(1, "zsh", 0.0, 0, "u"),
            make_proc(2, "bash", 0.0, 0, "u"),
            make_proc(3, "ash", 0.0, 0, "u"),
        ];
        apply_sort(
            &mut procs,
            &SortSpec {
                field: SortField::Name,
                direction: SortDirection::Asc,
            },
        );
        assert_eq!(procs[0].name, "ash");
        assert_eq!(procs[1].name, "bash");
        assert_eq!(procs[2].name, "zsh");
    }
}
